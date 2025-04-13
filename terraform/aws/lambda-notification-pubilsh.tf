resource "aws_iam_role" "notification_pubilsh" {
  name = "${terraform.workspace}-46ki75-notification-iam-role-notification-pubilsh"
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

resource "aws_iam_policy" "notification_pubilsh" {
  name        = "${terraform.workspace}-46ki75-internal-iam-policy-lambda-notification-pubilsh"
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
        ],
        "Resource" : "*"
      },
      {
        "Effect" : "Allow",
        "Action" : [
          "lambda:Invoke",
        ],
        "Resource" : "${aws_lambda_alias.notification_crud.arn}"
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "notification_pubilsh" {
  role       = aws_iam_role.notification_pubilsh.name
  policy_arn = aws_iam_policy.notification_pubilsh.arn
}

resource "aws_lambda_function" "notification_pubilsh" {
  function_name = "${terraform.workspace}-46ki75-notification-lambda-function-notification-pubilsh"
  role          = aws_iam_role.notification_pubilsh.arn
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

resource "aws_lambda_alias" "notification_pubilsh" {
  name             = "stable"
  function_name    = aws_lambda_function.notification_pubilsh.function_name
  function_version = "$LATEST"
}
