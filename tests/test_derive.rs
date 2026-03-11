use struct_input::StructInputTrait;
use struct_input_derive::StructInput;

#[derive(StructInput, Debug)]
struct TestStruct {
    #[struct_input(format="Name")]
    name: String,

    #[struct_input(format="BaseUrl")]
    email: String
}

#[tokio::test]
async fn test() {

    let test = TestStruct::from_input().await;
    println!("{:#?}", test);
}