pub mod handler {
    use lambda_runtime::LambdaEvent;
    use serde::Deserialize;
    use serde_json::{json, Value};

    #[derive(Deserialize)]
    pub struct LambdaEventPayload {
        pub first_name: String,
    }

    pub struct Response {
        pub result: String,
        pub user_hand: String,
        pub opponent_hand: String,
    }

    enum Hand {
        Rock,
        Paper,
        Scissors,
    }
    
    impl Hand {
        fn from_str(s: &str) -> Option<Hand> {
            match s {
                "rock" => Some(Hand::Rock),
                "paper" => Some(Hand::Paper),
                "scissors" => Some(Hand::Scissors),
                _ => None,
            }
        }
    
        fn to_str(&self) -> &str {
            match self {
                Hand::Rock => "rock",
                Hand::Paper => "paper",
                Hand::Scissors => "scissors",
            }
        }
    
        fn random() -> Hand {
            let mut rng = rand::thread_rng();
            match rng.gen_range(0..3) {
                0 => Hand::Rock,
                1 => Hand::Paper,
                2 => Hand::Scissors,
                _ => unreachable!(),
            }
        }
    
        fn judge(&self, other: &Hand) -> &'static str {
            match (self, other) {
                (Hand::Rock, Hand::Scissors) => "win",
                (Hand::Rock, Hand::Paper) => "lose",
                (Hand::Rock, Hand::Rock) => "draw",
                (Hand::Paper, Hand::Rock) => "win",
                (Hand::Paper, Hand::Scissors) => "lose",
                (Hand::Paper, Hand::Paper) => "draw",
                (Hand::Scissors, Hand::Paper) => "win",
                (Hand::Scissors, Hand::Rock) => "lose",
                (Hand::Scissors, Hand::Scissors) => "draw",
            }
        }
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
        // let first_name = payload.first_name;
        // ユーザーの手を入力
        let user_hand = Hand::from_str(&payload.first_name.to_lowercase())
            .ok_or(HandlerError("Invalid input"))?;
        // 相手の手をランダムに生成
        let opponent_hand = Hand::random();
        // 勝敗を判定
        let result = user_hand.judge(&opponent_hand);
        
        let response = Response {
            pub reult: format!("{reult}"),
            pub user_hand: format!("{user_hand}"),
            pub opponent_hand: format!("{opponent_hand}"),
        };

        Ok(json!({"reult": response.reult, "user_hand": response.user_hand, "opponent_hand": response.opponent_hand}))
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
    }
}
