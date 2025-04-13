// @see <https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/dynamodb_table>
resource "aws_dynamodb_table" "notification" {
  name = "${terraform.workspace}-46ki75-notification-dynamodb-table-notification"

  billing_mode   = "PROVISIONED"
  read_capacity  = 3
  write_capacity = 3

  hash_key = "PK"

  attribute {
    name = "PK"
    type = "S"
  }

  attribute {
    # Enum "NEW" | "OPEN" | "RESOLVED" | "SUPPRESSED" 
    name = "status"
    type = "S"
  }

  attribute {
    # RFC3339
    name = "notifiedAt"
    type = "S"
  }

  global_secondary_index {
    name            = "by-status"
    hash_key        = "status"
    range_key       = "notifiedAt"
    write_capacity  = 3
    read_capacity   = 3
    projection_type = "ALL"
  }
}
