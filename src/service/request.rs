fn requestSomething() {
  let client = Client::new();
  let respose = client.get("www.google.com").send().unwrap();
  if res.status == hyper::Ok {
    println!("success")
  }
}
