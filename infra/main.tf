
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

# TODO
resource "aws_s3_bucket_policy" "avatars_public" {
    provider = aws.rustfs

    bucket = aws_s3_bucket.avatars.id

    policy = jsonencode({
        version = ""
        Statement = [
            {
                Sid = "PublicRead"
                Effect = "Allow"

                Principal = "*"

                Action = [
                    "s3:GetObject"
                ]

                Resource = [
                    "${aws_s3_bucket.avatars.arn}/*"
                ]
            }
        ]
    })
}

# TODO
resource "aws_s3_bucket_cors_config" "avatars" {
    provider = aws.rustfs

    bucket = aws_s3_bucket.avatars.id

    cors_rule {
        allowed_methods = [
            "GET",
            "HEAD",
            "OPTIONS",
        ]

        allowed_origins = [
            "http://localhost:5173"
        ]

        allowed_headers = [
            "*"
        ]

        expose_headers = [
            "ETag"
        ]

        max_age_seconds = 3600
    }
}