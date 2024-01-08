# S3 On Object Versioning Enable Bucket

This edge test case is for AWS s3 service that has object versioning enabled.

For setup, please configure bucket correctly.
You need to create a bucket with object versioning enabled, for example:

```yaml
- name: Create bucket
  run: aws s3api create-bucket --bucket ${{ env.AWS_S3_BUCKET }} --region ${{ env.AWS_REGION }} --create-bucket-configuration LocationConstraint=${{ env.AWS_REGION }}
- name: Enable versioning
  run: aws s3api put-bucket-versioning --bucket ${{ env.AWS_S3_BUCKET }} --versioning-configuration Status=Enabled
```

Or you can create a bucket with object versioning enabled in the AWS console.

And configure `AWS_S3_BUCKET` and `AWS_REGION`.

