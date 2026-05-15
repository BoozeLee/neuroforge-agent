#!/usr/bin/env node
/**
 * NeuroForge SAP Registration via TypeScript SDK
 * Supports devnet and mainnet
 * 
 * Usage:
 *   node register-sdk.js devnet   # Register on devnet
 *   node register-sdk.js mainnet  # Register on mainnet
 */

const fs = require('fs');
const path = require('path');
const { exec } = require('child_process');
const { promisify } = require('util');

const execAsync = promisify(exec);

async function main() {
  const network = process.argv[2] || 'devnet';
  const sdkPath = '/tmp/synapse-sap-sdk';
  const keypairPath = path.join(__dirname, 'keys', 'agent.json');
  
  console.log('\n📋 NeuroForge SAP Registration');
  console.log('==============================');
  console.log('Network:', network.toUpperCase());
  
  // Check keypair exists
  if (!fs.existsSync(keypairPath)) {
    console.error('❌ Keypair not found at:', keypairPath);
    process.exit(1);
  }
  
  // Check SDK exists
  if (!fs.existsSync(sdkPath)) {
    console.error('❌ SDK not found at:', sdkPath);
    console.error('   Clone: git clone https://github.com/OOBE-PROTOCOL/synapse-sap-sdk.git /tmp/synapse-sap-sdk');
    process.exit(1);
  }
  
  try {
    // Load and display wallet
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf-8'));
    const { PublicKey, Keypair } = require(`${sdkPath}/node_modules/@solana/web3.js`);
    const keypair = Keypair.fromSecretKey(Buffer.from(keypairData));
    
    console.log('🔑 Wallet:', keypair.publicKey.toString());
    
    // Check balance
    const { Connection } = require(`${sdkPath}/node_modules/@solana/web3.js`);
    const sdk = require(`${sdkPath}/dist/cjs/index.js`);
    
    const endpoint = network === 'mainnet' ? sdk.ENDPOINTS.MAINNET : sdk.ENDPOINTS.DEVNET;
    const connection = new Connection(endpoint, 'confirmed');
    
    console.log('🔗 Connecting to:', endpoint);
    const balance = await connection.getBalance(keypair.publicKey);
    const solBalance = balance / 1e9;
    
    console.log('💰 Balance:', solBalance.toFixed(4), 'SOL');
    
    if (balance < 5000000) {
      console.error('\n❌ Insufficient balance (need 0.005 SOL minimum)');
      if (network === 'mainnet') {
        console.error('   Awaiting SOL from bounty team...');
      } else {
        console.error('   Use devnet faucet: https://faucet.solana.com/');
      }
      process.exit(1);
    }
    
    // Get correct PDAs
    const program = new PublicKey(sdk.PROGRAM_ID);
    const [agentPda] = sdk.Pdas.getAgentPDA(keypair.publicKey, program);
    const [statsPda] = sdk.Pdas.getAgentStatsPDA(keypair.publicKey, program);
    const [pricingPda] = sdk.Pdas.getEscrowV2PDA(keypair.publicKey, program);
    
    console.log('\n✅ Ready to register');
    console.log('📍 PDAs:');
    console.log('   Agent:', agentPda.toString());
    console.log('   Stats:', statsPda.toString());
    console.log('   Pricing:', pricingPda.toString());
    
    console.log('\n💡 Next steps:');
    if (network === 'devnet') {
      console.log('   1. Full SDK registration requires IDL setup');
      console.log('   2. Email oobe@oobeprotocol.ai for devnet support, or');
      console.log('   3. Wait for mainnet SOL and use: cargo run --bin register');
    } else {
      console.log('   Run: cargo run --bin register');
    }
    
  } catch (e) {
    console.error('❌ Error:', e.message);
    process.exit(1);
  }
}

main();
