const http = require("http");

const server = http.createServer((req, res) => {
  let body = "";

  req.on("data", (chunk) => {
    body += chunk;
  });

  req.on("end", () => {
    // Guard against empty body
    if (!body) {
      res.writeHead(400);
      res.end("END Bad request");
      return;
    }

    let parsed;
    try {
      parsed = JSON.parse(body);
    } catch (e) {
      console.error("Failed to parse body:", body);
      res.writeHead(400);
      res.end("END Bad request");
      return;
    }

    const { text } = parsed;
    console.log("Received request, text:", JSON.stringify(text));

    let response;

    if (text === "") {
      response = "CON Welcome to TestBank\n1. Check Balance\n2. Transfer\n3. Buy Airtime";
    } else if (text === "1") {
      response = "END Your balance is ₦25,000";
    } else if (text === "2") {
      response = "CON Enter account number:";
    } else if (text.startsWith("2*") && text.split("*").length === 2) {
      response = "CON Enter amount:";
    } else if (text.startsWith("2*") && text.split("*").length === 3) {
      response = "END Transfer successful!";
    } else if (text === "3") {
      response = "CON Enter phone number:";
    } else if (text.startsWith("3*")) {
      response = "END Airtime purchase successful!";
    } else {
      response = "END Invalid option selected.";
    }

    console.log("Sending response:", response);
    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end(response);
  });
});

server.listen(3000, () => {
  console.log("Test USSD app running on http://localhost:3000");
});