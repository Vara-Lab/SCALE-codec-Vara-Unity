using UnityEngine;
using System;
using System.Threading;
using System.Numerics;
using System.Text;
using Substrate.NetApi.Model.Extrinsics;
using VaraExt = Substrate.Vara.NET.NetApiExt.Generated;

public class ReadStateDecode : MonoBehaviour
{
    private VaraExt.SubstrateClientExt _clientvara;
    private string url;

    // Start is called before the first frame update
    async void Start()
    {
        // Assign the test node URL to the variable url
        url = "wss://testnet.vara.network";

        // Initialize the Substrate client with the node URL and the default transaction payment method
        _clientvara = new VaraExt.SubstrateClientExt(new Uri(url), ChargeTransactionPayment.Default());

        // Connect the client to the node asynchronously
        await _clientvara.ConnectAsync();

        // Check if the client is initialized and connected
        if (_clientvara != null && _clientvara.IsConnected)
        {
            // Log a message indicating that the client is connected
            Debug.Log("Client is connected.");

            // Define parameters for the RPC call
            string[] parametersArray = new string[] { "0x0b75f92b7454808b04e4b86ed7f4c3f53f45df147c19565e5b358fbc9a879172", "" };

            // Invoke the RPC method and get the result
            string text = await _clientvara.InvokeAsync<string>("gear_readState", parametersArray, CancellationToken.None);
             // Log the retrieved data to the debug console
            Debug.Log($"Data: {text}");


            // Convert the hex strings to byte arrays
            byte[] scaleBytes = StringToByteArray(text.Replace("0x", ""));

            // Decode the SCALE bytes
            Example example = Example.Decode(scaleBytes);
            Debug.Log(example);

            // Log the retrieved data to the debug console
            Debug.Log($"Data: {text}");
        }
        else
        {
            // Log a message indicating that the client is not connected
            Debug.Log("Client is not connected.");
        }
    }

    public static byte[] StringToByteArray(string hex)
    {
        int numberChars = hex.Length;
        byte[] bytes = new byte[numberChars / 2];
        for (int i = 0; i < numberChars; i += 2)
        {
            bytes[i / 2] = Convert.ToByte(hex.Substring(i, 2), 16);
        }
        Debug.Log("Converted bytes: " + BitConverter.ToString(bytes));
        return bytes;
    }

    [Serializable]
    public class Example
    {
        public uint Id { get; set; }
        public bool Active { get; set; }
        public string Description { get; set; }
        public BigInteger ExtraLargeNumber { get; set; }

        public static Example Decode(byte[] bytes)
        {
            int index = 0;

            // Decode uint32 (4 bytes, little-endian)
            if (index + 4 > bytes.Length)
                throw new ArgumentOutOfRangeException("Index and count must refer to a location within the buffer.");
            uint id = BitConverter.ToUInt32(bytes, index);
            index += 4;

            // Decode bool (1 byte)
            if (index + 1 > bytes.Length)
                throw new ArgumentOutOfRangeException("Index and count must refer to a location within the buffer.");
            bool active = bytes[index] == 1;
            index += 1;

            // Decode compact length-prefixed string
            int stringLength = DecodeCompactLength(bytes, ref index);
            if (index + stringLength > bytes.Length)
                throw new ArgumentOutOfRangeException("Index and count must refer to a location within the buffer.");
            string description = Encoding.UTF8.GetString(bytes, index, stringLength);
            index += stringLength;

            // Decode uint128
            if (index + 16 > bytes.Length)
                throw new ArgumentOutOfRangeException("Index and count must refer to a location within the buffer.");
            byte[] u128Bytes = new byte[16];
            Array.Copy(bytes, index, u128Bytes, 0, 16);
            Array.Reverse(u128Bytes); // Convert to Big-Endian for BigInteger
            BigInteger extraLargeNumber = new BigInteger(u128Bytes);
            index += 16;

            return new Example
            {
                Id = id,
                Active = active,
                Description = description,
                ExtraLargeNumber = extraLargeNumber
            };
        }

        private static int DecodeCompactLength(byte[] bytes, ref int index)
        {
            if (index + 1 > bytes.Length)
                throw new ArgumentOutOfRangeException("Index and count must refer to a location within the buffer.");
            byte firstByte = bytes[index];
            index += 1;

            int length;
            if ((firstByte & 0x03) == 0x00)
            {
                // Single byte
                length = firstByte >> 2;
            }
            else if ((firstByte & 0x03) == 0x01)
            {
                // Two bytes
                if (index + 1 > bytes.Length)
                    throw new ArgumentOutOfRangeException("Index and count must refer to a location within the buffer.");
                length = ((firstByte >> 2) | (bytes[index] << 6)) & 0x3FFF;
                index += 1;
            }
            else if ((firstByte & 0x03) == 0x02)
            {
                // Four bytes
                if (index + 3 > bytes.Length)
                    throw new ArgumentOutOfRangeException("Index and count must refer to a location within the buffer.");
                length = (firstByte >> 2) |
                         (bytes[index] << 6) |
                         (bytes[index + 1] << 14) |
                         (bytes[index + 2] << 22);
                index += 3;
            }
            else
            {
                throw new Exception("Unsupported compact length encoding");
            }

            return length;
        }

        public override string ToString()
        {
            return $"Id: {Id}, Active: {Active}, Description: {Description}, ExtraLargeNumber: {ExtraLargeNumber}";
        }
    }
}
