pub mod handler {
    use lambda_runtime::LambdaEvent;
    use rand::Rng;
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
    ) -> Result<Value, lambda_runtime::Error> {
        let payload = event.payload;
        let reult = execute(payload).await?;
        Ok(reult)
    }

    pub(crate) async fn execute(
        payload: LambdaEventPayload,
    ) -> Result<Value, lambda_runtime::Error> {
        // コンピュータの手をランダムに選択
        let choices = ["グー", "チョキ", "パー"];
        let mut rng = rand::thread_rng();
        let computer_choice = choices[rng.gen_range(0..choices.len())];

        let first_name = payload.first_name.as_str();

        // 勝負の結果を判定
        let result = match (first_name, computer_choice) {
            ("グー", "チョキ") | ("チョキ", "パー") | ("パー", "グー") => {
                "あなたの勝ちです！"
            }
            ("グー", "パー") | ("チョキ", "グー") | ("パー", "チョキ") => {
                "あなたの負けです。"
            }
            _ => "引き分けです。",
        };

        let response = Response {
            message: format!("あなたは{first_name}、私は{computer_choice}"),
        };

        Ok(
            json!({"user_choice": first_name, "computer_choice":computer_choice, "result": result, "message": response.message}),
        )
    }

    #[cfg(test)]
    mod test {

        use super::*;

        #[tokio::test]
        async fn handler_test() {
            let name = LambdaEventPayload {
                first_name: "ぐー".to_string(),
            };
            let result = execute(name).await;
            assert!(result.is_ok());
        }
    }
}
