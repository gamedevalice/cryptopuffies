/* eslint-disable @typescript-eslint/no-explicit-any */
import { useState, useEffect, createContext, PropsWithChildren, useContext, useCallback } from 'react'

import detectEthereumProvider from '@metamask/detect-provider'
import { formatBalance } from '~/utils'
import { ethers } from 'ethers'

interface WalletState {
  accounts: any[]
  balance: string
  chainId: string
  puffyTotalSupply: string
  puffyBalanceOfSelf: string
  puffyTokensOfSelf: string[],
  cryptoPuffiesBestFriendsContract: ethers.Contract | null
  cryptoPuffiesContractWriteable: ethers.Contract | null
}


interface MetaMaskContextData {
  wallet: WalletState
  hasProvider: boolean | null
  error: boolean
  errorMessage: string
  isConnecting: boolean
  connectMetaMask: () => void
  clearError: () => void
}

const disconnectedState: WalletState = {
  accounts: [], balance: '', chainId: '',
  puffyTotalSupply: '',
  puffyBalanceOfSelf: '',
  puffyTokensOfSelf: [],
  cryptoPuffiesBestFriendsContract: null,
  cryptoPuffiesContractWriteable: null
}

const MetaMaskContext = createContext<MetaMaskContextData>({} as MetaMaskContextData)

export const MetaMaskContextProvider = ({ children }: PropsWithChildren) => {
  const [hasProvider, setHasProvider] = useState<boolean | null>(null)

  const [isConnecting, setIsConnecting] = useState(false)

  const [errorMessage, setErrorMessage] = useState('')
  const clearError = () => setErrorMessage('')

  const [wallet, setWallet] = useState(disconnectedState)
  // useCallback ensures that we don't uselessly re-create the _updateWallet function on every render
  const _updateWallet = useCallback(async (providedAccounts?: any) => {
    const accounts = providedAccounts || await window.ethereum.request(
      { method: 'eth_accounts' },
    )

    if (accounts.length === 0) {
      // If there are no accounts, then the user is disconnected
      setWallet(disconnectedState)
      return
    }

    const balance = formatBalance(await window.ethereum.request({
      method: 'eth_getBalance',
      params: [accounts[0], 'latest'],
    }))
    const chainId = await window.ethereum.request({
      method: 'eth_chainId',
    })
    
    //load contracts
    let signer = null;
    let provider;
    if (window.ethereum == null) {
        console.log("MetaMask not installed; using read-only defaults")
        provider = ethers.getDefaultProvider("https://api.avax.network/ext/bc/C/rpc")
    } else {
        provider = new ethers.BrowserProvider(window.ethereum)
        signer = await provider.getSigner();
    }
    var cryptoPuffiesAbi = [
      "function totalSupply() view returns (uint)",
      "function balanceOf(address addr) view returns (uint)",
      "function tokensOfOwner(address addr) view returns (uint[])",
      "function mint(uint amount)"
    ]
    var cryptoPuffiesContract = new ethers.Contract("0x457C224e4A2db059071F01ee2Ff43078Ac021597", cryptoPuffiesAbi, provider)
    var cryptoPuffiesContractWriteable = new ethers.Contract("0x457C224e4A2db059071F01ee2Ff43078Ac021597", cryptoPuffiesAbi, signer)

    var cryptoPuffiesBestFriendsAbi = [
      "function bestFriend(uint id) view returns (uint)",
      "function nemesis(uint id) view returns (uint)"
    ]
    var cryptoPuffiesBestFriendsContract = new ethers.Contract("0xb55B9A49E7f3A888e29C15a02C1b044826457Bd0", cryptoPuffiesBestFriendsAbi, provider)
    
    const selfAddress = accounts[0].toString();
    const puffyTotalSupply = (await cryptoPuffiesContract.totalSupply()).toString()
    const puffyBalanceOfSelf = (await cryptoPuffiesContract.balanceOf(selfAddress)).toString()
    const puffyTokensOfSelf: string[] = (await cryptoPuffiesContract.tokensOfOwner(selfAddress))

    setWallet({ accounts, balance, chainId, puffyTotalSupply, puffyBalanceOfSelf, puffyTokensOfSelf, cryptoPuffiesBestFriendsContract, cryptoPuffiesContractWriteable })
  }, [])

  const updateWalletAndAccounts = useCallback(() => _updateWallet(), [_updateWallet])
  const updateWallet = useCallback((accounts: any) => _updateWallet(accounts), [_updateWallet])

  /**
   * This logic checks if MetaMask is installed. If it is, then we setup some
   * event handlers to update the wallet state when MetaMask changes. The function
   * returned from useEffect is used as a "clean-up": in there, we remove the event
   * handlers whenever the MetaMaskProvider is unmounted.
   */
  useEffect(() => {
    const getProvider = async () => {
      const provider = await detectEthereumProvider({ silent: true })
      setHasProvider(Boolean(provider))

      if (provider) {
        updateWalletAndAccounts()
        window.ethereum.on('accountsChanged', updateWallet)
        window.ethereum.on('chainChanged', updateWalletAndAccounts)
      }
    }

    getProvider()

    return () => {
      window.ethereum?.removeListener('accountsChanged', updateWallet)
      window.ethereum?.removeListener('chainChanged', updateWalletAndAccounts)
    }
  }, [updateWallet, updateWalletAndAccounts])

  const connectMetaMask = async () => {
    setIsConnecting(true)

    try {
      const accounts = await window.ethereum.request({
        method: 'eth_requestAccounts',
      })
      clearError()
      updateWallet(accounts)
    } catch(err: any) {
      setErrorMessage(err.message)
    }
    setIsConnecting(false)
  }

  return (
    <MetaMaskContext.Provider
      value={{
        wallet,
        hasProvider,
        error: !!errorMessage,
        errorMessage,
        isConnecting,
        connectMetaMask,
        clearError,
      }}
    >
      {children}
    </MetaMaskContext.Provider>
  )
}

export const useMetaMask = () => {
  const context = useContext(MetaMaskContext)
  if (context === undefined) {
    throw new Error('useMetaMask must be used within a "MetaMaskContextProvider"')
  }
  return context
}