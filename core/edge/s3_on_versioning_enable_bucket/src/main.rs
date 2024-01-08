// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use opendal::raw::tests::init_test_service;
use opendal::Result;
use opendal::Scheme;
use rand::prelude::*;
use uuid::Uuid;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let op = init_test_service()?.expect("service must be init");
    assert_eq!(op.info().scheme(), Scheme::S3);

    let path = Uuid::new_v4().to_string();
    let path = path.as_ref();
    let max_size = op
        .info()
        .full_capability()
        .write_total_max_size
        .unwrap_or(4 * 1024 * 1024);

    let mut rng = thread_rng();

    let size = rng.gen_range(1..max_size);
    let mut first_content = vec![0; size];
    rng.fill_bytes(&mut first_content);

    // Write the first content
    op.write(path, first_content.clone())
        .await
        .expect("write must success");

    let mut versions: Vec<&str> = vec![];

    // To confirm the version is not empty
    let meta = op.stat(path).await.expect("meta must success");
    assert!(meta.version().is_some(), "version must be some");
    versions.push(meta.version().unwrap());

    // Read the first content
    let content = op.read(path).await.expect("read must success");
    assert_eq!(content, first_content, "content must be equal");

    let size = rng.gen_range(1..max_size);
    let mut second_content = vec![0; size];
    rng.fill_bytes(&mut second_content);

    // Write the second content
    op.write(path, second_content.clone())
        .await
        .expect("write must success");
    let meta = op.stat(path).await.expect("meta must success");
    assert!(meta.version().is_some(), "version must be some");
    versions.push(meta.version().unwrap());

    // There must be two versions, and they must be not equal
    assert_ne!(versions[0], versions[1], "versions must be not equal");

    // Read the second content, and it must be equal to the second content
    // and not equal to the first content
    let content = op.read(path).await.expect("read must success");
    assert_ne!(first_content, content, "content must be not equal");
    assert_eq!(second_content, content, "content must be equal");

    // Delete the path
    op.delete(path).await.expect("delete must success");
    let meta = op.stat(path).await;
    assert!(meta.is_err(), "stat must return error");
    assert_eq!(
        meta.err().unwrap().kind(),
        opendal::ErrorKind::NotFound,
        "error kind must be not found"
    );

    // Read the first content with the first version, and it must be equal to the first content
    let content = op
        .read_with(path)
        .version(versions[0])
        .await
        .expect("read must success");
    assert_eq!(first_content, content, "content must be equal");

    // Delete the path with the first version
    let meta = op.stat_with(path).version(versions[0]).await;
    assert!(meta.is_err(), "stat must return error");
    assert_eq!(
        meta.err().unwrap().kind(),
        opendal::ErrorKind::NotFound,
        "error kind must be not found"
    );

    // Finally, delete the path with the second version
    op.delete_with(path)
        .version(versions[1])
        .await
        .expect("delete must success");

    Ok(())
}
