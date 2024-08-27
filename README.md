# Serverless API with Rust 

This project demonstrates how to build a serverless API using Rust. It leverages AwS Lambda and API Gateway to create a scalable and efficient serverless application.

## Getting Started 
### Prerequisites 
- **Rust:** [Install Rust](https://www.rust-lang.org/learn/get-started) if you haven't already.
- **AWS CLI:** [Install AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html) to manage AWS services.
- **AWS SAM CLI:** [Install AWS SAM CLI](https://docs.aws.amazon.com/serverless/latest/dg/install-sam-cli.html) for deploying the serverless application.

### Feature 
- **AWS Lambda Function**: The core of the serverless architecture, handling the business logic and processing requests.
- **API Gateway:** Acts as the entry point for HTTP requests, routing them to the appropriate Lambda functions.
- **Example Endpoints:** Demonstrates common API operations, such as GET,POST,PUT and DELETE, with sample payloads.
- **Deployment Automation:** Scripts and templates to deploy the API using AWS CloundFormation or the Serverless Framework.

### Build and Deploy
1. **Build the Application:**

Navigate to the root directory of the project where the `template.yaml` file is located. Run the following commands to build SAM application:
```bash 
sam build
``` 

2. **Deploy the Application:**

After building the application, deploy it using the following command:

```bash

sam deploy --guilded

```

After providing the necessary information, SAM CLI will package and deploy your application. The process includes 
creating or updating AWS Lambda functions, API Gateway endpoints, and other AWS resources defined in your `template.yaml` file

3. **Verify Deployment:**

Once the deployment is complete, SAM CLI will output the URL of your deployed API Gateway endpoint.

*Note:* Replace `<your-endpoint>` with the actual endpoint provided by SAM CLI. 

- Create Todo 
```bash 
http POST https://<your-endpoint>.amazonaws.com/Prod/todo/ Content-Type:application/json --raw '{"id": "11", "title": "Todo", "completed": false}'
```
- Get Todo
```bash
http GET https:/<your-endpoint>.amazonaws.com/Prod/todo/1 Content-Type:application/json
```

- Update Todo
```bash 
http PUT https://<your-endpoint>.amazonaws.com/Prod/todo/ Content-Type:application/json --raw '{"id": "11", "title": "Update Todo", "completed": false}'
```

- Delete Todo
```bash
http DELETE https:/<your-endpoint>.amazonaws.com/Prod/todo/11 Content-Type:application/json
```


*Note:*  for testing purposes, it is  recommended  to use `Stage` environment instead of `Prod`


### Clean Up 
To remove the deployed resources, you can delete CloudFormation stack:
```

sam delete
```

This command will prompt you for comfirmation and then delete the stack along with all associated resources.
