import { useMetaMask } from '~/hooks/useMetaMask'
import { formatAddress } from '~/utils'
import styles from './Navigation.module.css'
import banner from '../../assets/Puffies-NFT-Collection-banner.jpg';

export const Navigation = () => {

  const { wallet, hasProvider, isConnecting, connectMetaMask } = useMetaMask()

  return (
    <div className={styles.navigation}>
      <div>
        
      </div>
      <div className={styles.flexContainer}>
        <div className={styles.leftNav}><b>CryptoPuffies:</b> Revived and maintained by GamedevAlice</div>
        <div className={styles.rightNav}>
          {!hasProvider &&
            <a href="https://metamask.io" target="_blank" rel="noreferrer">
              Install MetaMask
            </a>
          }
          {window.ethereum?.isMetaMask && wallet.accounts.length < 1 &&
            <button disabled={isConnecting} onClick={connectMetaMask}>
              Connect MetaMask
            </button>
          }
          {hasProvider && wallet.accounts.length > 0 &&
            <a
              className="text_link tooltip-bottom"
              href={`https://etherscan.io/address/${wallet}`}
              target="_blank"
              data-tooltip="Open in Block Explorer" rel="noreferrer"
            >
              {formatAddress(wallet.accounts[0])}
            </a>
          }
        </div>
      </div>
          <img className={styles.banner} src={banner} alt="Banner" />
    </div>
  )
}