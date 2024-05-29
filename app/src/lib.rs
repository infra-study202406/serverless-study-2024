pub mod handler {
    use lambda_runtime::LambdaEvent;
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[derive(Deserialize)]
    pub struct LambdaEventPayload {
        pub first_name: String,
    }

    pub struct Response {
        pub message: String,
    }

    pub async fn lamdba_handler(
        event: LambdaEvent<LambdaEventPayload>,
    ) -> Result<(), lambda_runtime::Error> {
        let payload = event.payload;
        execute(payload).await?;
        Ok(())
    }

    pub(crate) async fn execute(
        payload: LambdaEventPayload,
    ) -> Result<Value, lambda_runtime::Error> {
        let first_name = payload.first_name;
        let response = Response {
            message: format!("Hello, {first_name}!"),
        };

        Ok(json!({"message": response.message}))
    }

    #[cfg(test)]
    mod test {

        use super::*;

        #[tokio::test]
        async fn handler_test() {
            let name = LambdaEventPayload {
                first_name: "test_man".to_string(),
            };
            let result = execute(name).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn handler_test_tanoue() {
            let name = LambdaEventPayload {
                first_name: 1,
            };
            let result = execute(name).await;
            assert!(result.is_ok());
        }
    }
}
