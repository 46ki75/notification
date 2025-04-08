terraform {
  required_providers {
    github = {
      source  = "hashicorp/aws"
      version = "~> 5.76"
    }
  }
  
  backend "s3" {
    bucket               = "shared-46ki75-notification-s3-bucket-terraform-tfstate"
    workspace_key_prefix = "aws"
    key                  = "terraform.tfstate"
    region               = "ap-northeast-1"
    encrypt              = true
    use_lockfile         = true
  }
}
