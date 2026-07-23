
terraform {
    required_providers {
        aws = {
            source = "hashicorp/aws"
            version = "~> 6.0"
        }
    }
}

provider "aws" {
    alias = "rustfs"
    region = "eu-west-2"
    access_key = var.access_key
    secret_key = var.secret_key
    endpoints {
        s3 = var.endpoint
    }

    skip_credentials_validation = true
    skip_metadata_api_check = true
    skip_requesting_account_id = true
    s3_use_path_style = true
}

resource "aws_s3_bucket" "avatars" {
    provider = aws.rustfs
    bucket = "avatars"
}

resource "aws_s3_bucket_policy" "avatars_public" {
    provider = aws.rustfs
    bucket = aws_s3_bucket.avatars.id
    policy = jsonencode({
        Version = "2012-10-17"
        Statement = [
            {
                Sid = "PublicReadGetObject"
                Effect = "Allow"
                Principal = {
                    AWS = ["*"]
                }
                Action = [
                    "s3:GetObject"
                ]
                Resource = [
                    "${aws_s3_bucket.avatars.arn}/*"
                ]
            },
            {
                Sid = "PublicListBucket"
                Effect = "Allow"
                Principal = {
                    AWS = ["*"]
                }
                Action = [
                    "s3:ListBucket"
                ]
                Resource = [
                    "arn:aws:s3:::avatars"
                ]
            }
        ]
    })
}
