resource "aws_iam_role" "notification_crud" {
  name = "${terraform.workspace}-46ki75-notification-iam-role-notification-crud"
  assume_role_policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Effect" : "Allow",
        "Principal" : {
          "Service" : "lambda.amazonaws.com"
        },
        "Action" : "sts:AssumeRole"
      }
    ]
  })
}

resource "aws_iam_policy" "notification_crud" {
  name        = "${terraform.workspace}-46ki75-internal-iam-policy-lambda-notification-crud"
  description = "Allow lambda to access cloudwatch logs"
  policy = jsonencode({
    "Version" : "2012-10-17",
    "Statement" : [
      {
        "Effect" : "Allow",
        "Action" : [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents",
          "xray:PutTraceSegments",
          "xray:PutTelemetryRecords",
          "dynamodb:Query",
          "dynamodb:GetItem",
          "dynamodb:PutItem",
          "dynamodb:DeleteItem",
        ],
        "Resource" : "*"
      }
    ]
  })
}

resource "aws_lambda_function" "notification_crud" {
  function_name = "${terraform.workspace}-46ki75-notification-lambda-function-notification-crud"
  role          = aws_iam_role.notification_crud.arn
  filename      = "./assets/bootstrap.zip"
  handler       = "bootstrap.handler"
  runtime       = "provided.al2023"
  architectures = ["x86_64"]
  publish       = true # Publish a new version
  timeout       = 3

  logging_config {
    # log_group             = aws_cloudwatch_log_group.
    log_format            = "JSON"
    application_log_level = "DEBUG"
    system_log_level      = "INFO"
  }

  tracing_config {
    mode = "Active"
  }

  environment {
    variables = {
      STAGE_NAME = terraform.workspace
    }
  }
}

resource "aws_lambda_alias" "graphql" {
  name             = "stable"
  function_name    = aws_lambda_function.notification_crud.function_name
  function_version = "$LATEST"
}
