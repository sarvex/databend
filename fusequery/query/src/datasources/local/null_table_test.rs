// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_null_table() -> anyhow::Result<()> {
    use common_datavalues::*;
    use common_planners::*;
    use futures::TryStreamExt;

    use crate::datasources::local::*;

    let ctx = crate::tests::try_create_context()?;
    let table = NullTable::try_create(
        ctx.clone(),
        "default".into(),
        "a".into(),
        DataSchema::new(vec![DataField::new("a", DataType::UInt64, false)]).into(),
        TableOptions::default(),
    )?;
    table.read_plan(ctx.clone(), PlanBuilder::empty().build()?)?;
    assert_eq!(table.engine(), "Null");

    let stream = table.read(ctx).await?;
    let blocks = stream.try_collect::<Vec<_>>().await?;
    let rows: usize = blocks.iter().map(|block| block.num_rows()).sum();

    assert_eq!(rows, 0);
    Ok(())
}
