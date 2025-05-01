"""
Industrial Control Example
This module demonstrates a simulated industrial control system using IMCP protocol.
It includes a server managing multiple industrial devices and a client for monitoring and control.
"""

import asyncio
import random
from datetime import datetime
from loguru import logger
from imcp.client import IMCPClient
from imcp.server import IMCPServer

class IndustrialDevice:
    """
    Simulated industrial device that maintains state and metrics.
    
    Attributes:
        device_id (str): Unique identifier for the device
        temperature (float): Current temperature reading (20.0-30.0°C)
        pressure (float): Current pressure reading (0.5-2.0 bar)
        status (str): Current device status (IDLE/RUNNING/STOPPED/MAINTENANCE)
        last_maintenance (datetime): Timestamp of last maintenance
    """
    def __init__(self, device_id: str):
        """
        Initialize a new industrial device with default values.
        
        Args:
            device_id (str): Unique identifier for the device
        """
        self.device_id = device_id
        self.temperature = 25.0  # Initial temperature in Celsius
        self.pressure = 1.0      # Initial pressure in bar
        self.status = "IDLE"     # Initial device status
        self.last_maintenance = datetime.now()
        logger.info(f"Device {device_id} initialized")

    def update_metrics(self):
        """
        Simulate real-time updates to device metrics.
        Randomly adjusts temperature and pressure within safe operating ranges.
        Temperature range: 20.0-30.0°C
        Pressure range: 0.5-2.0 bar
        """
        self.temperature += random.uniform(-0.5, 0.5)
        self.pressure += random.uniform(-0.1, 0.1)
        # Ensure metrics stay within safe operating ranges
        self.temperature = max(20.0, min(30.0, self.temperature))
        self.pressure = max(0.5, min(2.0, self.pressure))
        logger.debug(f"Device {self.device_id} metrics updated")

    def get_metrics(self):
        """
        Get current device metrics and status.
        
        Returns:
            dict: Dictionary containing device metrics and status
        """
        return {
            "device_id": self.device_id,
            "temperature": self.temperature,
            "pressure": self.pressure,
            "status": self.status,
            "last_maintenance": self.last_maintenance.isoformat()
        }

async def run_industrial_server():
    """
    Run the industrial control server that manages multiple devices.
    
    The server:
    1. Initializes simulated devices
    2. Sets up IMCP server
    3. Handles incoming messages for device monitoring and control
    """
    # Initialize simulated devices
    devices = {
        "DEV001": IndustrialDevice("DEV001"),
        "DEV002": IndustrialDevice("DEV002"),
        "DEV003": IndustrialDevice("DEV003")
    }
    logger.info("Simulated devices initialized")
    
    # Initialize IMCP server
    server = IMCPServer(host="localhost", port=8767)
    logger.info("Starting industrial control server...")

    async def handle_industrial_message(message, client_public_key):
        """
        Handle incoming industrial control messages.
        
        Args:
            message (dict): The received message
            client_public_key: Client's public key for authentication
            
        Returns:
            dict: Response containing device metrics or control status
        """
        if message["header"]["type"] == "DATA":
            # Handle data requests (metrics retrieval)
            device_id = message["body"]["device_id"]
            if device_id in devices:
                device = devices[device_id]
                device.update_metrics()
                return device.get_metrics()
        elif message["header"]["type"] == "CONTROL":
            # Handle control commands
            command = message["body"]["command"]
            device_id = message["body"]["device_id"]
            if device_id in devices:
                device = devices[device_id]
                if command == "START":
                    device.status = "RUNNING"
                elif command == "STOP":
                    device.status = "STOPPED"
                elif command == "MAINTENANCE":
                    device.status = "MAINTENANCE"
                    device.last_maintenance = datetime.now()
                logger.info(f"Device {device_id} status changed to {device.status}")
                return {"status": "success", "device_status": device.status}
        return {"status": "error", "message": "Invalid command"}

    server.handle_message = handle_industrial_message
    await server.start()

async def run_industrial_client():
    """
    Run the industrial control client that monitors and controls devices.
    
    The client:
    1. Connects to the server
    2. Monitors device metrics
    3. Sends control commands when needed
    4. Schedules maintenance
    """
    client = IMCPClient("ws://localhost:8767")
    
    try:
        # Connect to server
        await client.connect()
        logger.info("Connected to industrial control server")

        # Monitor device metrics
        for device_id in ["DEV001", "DEV002", "DEV003"]:
            # Request device metrics
            await client.send_control(
                "GET_METRICS",
                {"device_id": device_id}
            )
            metrics = await client.receive()
            logger.info(f"Device {device_id} metrics: {metrics}")

            # Send control commands if temperature exceeds threshold
            if metrics["temperature"] > 28.0:
                await client.send_control(
                    "CONTROL",
                    {
                        "device_id": device_id,
                        "command": "STOP"
                    }
                )
                response = await client.receive()
                logger.info(f"Device {device_id} control response: {response}")

        # Schedule maintenance for DEV001
        await client.send_control(
            "CONTROL",
            {
                "device_id": "DEV001",
                "command": "MAINTENANCE"
            }
        )
        response = await client.receive()
        logger.info("Maintenance scheduled for DEV001")

    except Exception as e:
        logger.error(f"Industrial client error: {e}")
    finally:
        await client.close()

async def main():
    """
    Main function to run the industrial control example.
    
    The function:
    1. Starts the industrial server in background
    2. Waits for server initialization
    3. Runs the industrial client
    4. Properly shuts down the server
    """
    # Start industrial server in background
    server_task = asyncio.create_task(run_industrial_server())
    
    # Wait for server to start
    await asyncio.sleep(1)
    
    # Run industrial client
    await run_industrial_client()
    
    # Stop server
    server_task.cancel()
    try:
        await server_task
    except asyncio.CancelledError:
        pass

if __name__ == "__main__":
    # Configure logging
    logger.add("imcp_industrial_example.log", rotation="1 MB", level="INFO")
    
    # Run the example
    asyncio.run(main()) 