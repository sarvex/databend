// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_ast::ast::Expr;
use common_ast::ast::TableReference;
use common_exception::ErrorCode;
use common_exception::Result;

use super::contain_subquery;
use crate::binder::Binder;
use crate::binder::ScalarBinder;
use crate::optimizer::SExpr;
use crate::optimizer::SubqueryRewriter;
use crate::plans::DeletePlan;
use crate::plans::Filter;
use crate::plans::Plan;
use crate::BindContext;

impl<'a> Binder {
    pub(in crate::planner::binder) async fn bind_delete(
        &mut self,
        bind_context: &BindContext,
        table_reference: &'a TableReference,
        filter: &'a Option<Expr>,
    ) -> Result<Plan> {
        let (catalog_name, database_name, table_name) = if let TableReference::Table {
            catalog,
            database,
            table,
            ..
        } = table_reference
        {
            (
                catalog
                    .as_ref()
                    .map_or_else(|| self.ctx.get_current_catalog(), |i| i.name.clone()),
                database
                    .as_ref()
                    .map_or_else(|| self.ctx.get_current_database(), |i| i.name.clone()),
                table.name.clone(),
            )
        } else {
            // we do not support USING clause yet
            return Err(ErrorCode::Internal(
                "should not happen, parser should have report error already",
            ));
        };

        let (table_expr, context) = self
            .bind_table_reference(bind_context, table_reference)
            .await?;

        let mut scalar_binder = ScalarBinder::new(
            &context,
            self.ctx.clone(),
            &self.name_resolution_ctx,
            self.metadata.clone(),
            &[],
        );

        let (selection, input_expr) = if let Some(expr) = filter {
            let (scalar, _) = scalar_binder.bind(expr).await?;
            if contain_subquery(&scalar) {
                let filter = Filter {
                    predicates: vec![scalar],
                    is_having: false,
                };
                let mut filter_expr = SExpr::create_unary(filter.into(), table_expr);
                let mut rewriter = SubqueryRewriter::new(self.metadata.clone());
                rewriter.rewrite(&mut filter_expr)?;
                (None, Some(filter_expr))
            } else {
                (Some(scalar), None)
            }
        } else {
            (None, None)
        };

        let plan = DeletePlan {
            catalog_name,
            database_name,
            table_name,
            selection,
            input_expr,
        };
        Ok(Plan::Delete(Box::new(plan)))
    }
}
