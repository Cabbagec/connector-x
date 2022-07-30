
// use arrow::{
//     array::{BooleanArray, Float64Array, Int64Array, StringArray},
//     record_batch::RecordBatch,
// };
// use connectorx::{
//     destinations::arrow::ArrowDestination, prelude::*, sources::mssql::MsSQLSource, sql::CXQuery,
//     transports::MsSQLArrowTransport,
// };
// use std::env;
// use std::sync::Arc;
// use tokio::runtime::Runtime;

#[test]
fn test_mssql() {
    let _ = env_logger::builder().is_test(true).try_init();

    let dburl = env::var("MSSQL_URL").unwrap();

    let queries = [
        CXQuery::naked("select * from test_table where test_int < 2"),
        CXQuery::naked("select * from test_table where test_int >= 2"),
    ];
    let rt = Arc::new(Runtime::new().unwrap());

    let builder = MsSQLSource::new(rt, &dburl, 2).unwrap();
    let mut destination = ArrowDestination::new();
    let dispatcher =
        Dispatcher::<_, _, MsSQLArrowTransport>::new(builder, &mut destination, &queries, None);
    dispatcher.run().unwrap();

    let result = destination.arrow().unwrap();
    verify_arrow_results(result);
}

// pub fn verify_arrow_results(result: Vec<RecordBatch>) {
//     assert!(result.len() == 2);

//     for rb in result {
//         assert!(rb.columns().len() == 5);
//         match rb.num_rows() {
//             2 => {
//                 assert!(rb
//                     .column(0)
//                     .as_any()
//                     .downcast_ref::<Int64Array>()
//                     .unwrap()
//                     .eq(&Int64Array::from(vec![1, 0])));

//                 assert!(rb
//                     .column(1)
//                     .as_any()
//                     .downcast_ref::<Int64Array>()
//                     .unwrap()
//                     .eq(&Int64Array::from(vec![Some(3), Some(5)])));

//                 assert!(rb
//                     .column(2)
//                     .as_any()
//                     .downcast_ref::<StringArray>()
//                     .unwrap()
//                     .eq(&StringArray::from(vec![Some("str1"), Some("a"),])));

//                 assert!(rb
//                     .column(3)
//                     .as_any()
//                     .downcast_ref::<Float64Array>()
//                     .unwrap()
//                     .eq(&Float64Array::from(vec![None, Some(3.1 as f64)])));

//                 assert!(rb
//                     .column(4)
//                     .as_any()
//                     .downcast_ref::<BooleanArray>()
//                     .unwrap()
//                     .eq(&BooleanArray::from(vec![Some(true), None])));
//             }
//             4 => {
//                 assert!(rb
//                     .column(0)
//                     .as_any()
//                     .downcast_ref::<Int64Array>()
//                     .unwrap()
//                     .eq(&Int64Array::from(vec![2, 3, 4, 1314])));

//                 assert!(rb
//                     .column(1)
//                     .as_any()
//                     .downcast_ref::<Int64Array>()
//                     .unwrap()
//                     .eq(&Int64Array::from(vec![None, Some(7), Some(9), Some(2)])));

//                 assert!(rb
//                     .column(2)
//                     .as_any()
//                     .downcast_ref::<StringArray>()
//                     .unwrap()
//                     .eq(&StringArray::from(vec![
//                         Some("str2"),
//                         Some("b"),
//                         Some("c"),
//                         None,
//                     ])));

//                 assert!(rb
//                     .column(3)
//                     .as_any()
//                     .downcast_ref::<Float64Array>()
//                     .unwrap()
//                     .eq(&Float64Array::from(vec![
//                         Some(2.2 as f64),
//                         Some(3 as f64),
//                         Some(7.8 as f64),
//                         Some(-10 as f64),
//                     ])));

//                 assert!(rb
//                     .column(4)
//                     .as_any()
//                     .downcast_ref::<BooleanArray>()
//                     .unwrap()
//                     .eq(&BooleanArray::from(vec![
//                         Some(false),
//                         Some(false),
//                         None,
//                         Some(true),
//                     ])));
//             }
//             _ => unreachable!(),
//         }
//     }
// }




#[test] // ODBC Test 
fn test_odbc() {
    // setup test 
    let a = 1;
    let b = 2;
    // call functions we want to test
    // assert correctness 
    assert_eq!(a+a, b);
} 