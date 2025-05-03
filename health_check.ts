// Define the Data interface similar to your Rust struct
interface Data {
  status: number;
  success: boolean;
  message: string;
}

// Use Node.js fs module
import * as fs from 'fs';

// Function to read the health check response from the JSON file
function readHealthCheckResponse(): Data {
  try {
    // Read the JSON file created by the Rust program
    const responseText = fs.readFileSync('health_check_response.json', 'utf8');
    
    // Log the raw response
    console.log('Raw response from Rust:');
    console.log(responseText);
    
    // Parse the JSON
    const data: Data = JSON.parse(responseText);
    
    // Log the parsed data
    console.log('Parsed data:');
    console.log(`Status: ${data.status}, Success: ${data.success}, Message: ${data.message}`);
    
    return data;
  } catch (e) {
    console.error(`Failed to read health check response: ${e}`);
    throw e;
  }
}

// Read and log the health check response
try {
  const data = readHealthCheckResponse();
  console.log('Health check response successfully read from Rust!');
} catch (e) {
  console.error(`Error reading health check response: ${e}`);
}
