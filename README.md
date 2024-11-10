VeilPiercer
===========

VeilPiercer is a powerful IP exposure tool designed to reveal the hidden IP addresses behind Cloudflare-protected domains. With VeilPiercer, you can cut through the protective veil, exposing the true origin server IP along with detailed geolocation and ISP data.

Features
--------

*   **IP Exposure**: Resolves and reveals the real IP address behind a Cloudflare-protected domain.
    
*   **Geolocation Insight**: Fetches the country, city, region, and ISP details of the exposed IP.
    
*   **Dual Data Sources**: Cross-references IP data from ipinfo.io and ip-api.com for accuracy.
    

> **Warning:** This tool is for ethical use only. Unauthorized use may violate terms of service and local laws.

Installation
------------

git clone https://github.com/yourusername/veilpiercer.gitcd veilpiercer

odecurl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

bashCopy codecargo build --release

Usage
-----

Run VeilPiercer with a target domain to uncover the real IP.

` codecargo run --release --` 

**Example:**

 `codecargo run --release -- example.com`

### Output

VeilPiercer will display the following information:

*   **Target Domain**: The domain you queried.
    
*   **Resolved Cloudflare IP**: The IP address Cloudflare shows.
    
*   **Real IP**: The exposed origin IP address.
    
*   **Geolocation**: Country, region, city, ISP, and timezone data of the origin server.
    

Configuration
-------------

The tool uses the ipinfo.io and ip-api.com APIs. Ensure you have a valid ipinfo.io token if required. Update it in src/main.rs:

`let token = "YOUR_TOKEN_HERE"; // Replace with your own token if needed`

Example Output
--------------

Target Domain: example.com  Resolved Cloudflare IP: 104.27.142.85  --- IPInfo ---  IP: 192.168.1.1  Country: United States  Region: California  City: Los Angeles  Organization: Example ISP  --- IP-API ---  Country: United States  Region: California  City: Los Angeles  ISP: Example ISP  Query IP: 192.168.1.1   `

Legal Disclaimer
----------------

VeilPiercer is intended for educational and ethical testing purposes only. Unauthorized use of this tool to bypass security measures is prohibited and may be illegal. The developer is not responsible for any misuse or damage caused by this tool.
