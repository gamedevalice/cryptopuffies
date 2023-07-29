import { useMetaMask } from '~/hooks/useMetaMask'
import { formatChainAsNum } from '~/utils'
import styles from './Display.module.css'
import React, { useEffect, useState } from 'react';
import { ethers } from "ethers";
import tokenData from '../../../docs/token-data.json';

export const Display = () => {

  const { wallet } = useMetaMask()
  var [puffyToPreview, setPuffyToPreview] = useState(wallet.puffyTotalSupply);

  async function clickMint(): Promise<void> {
    if (wallet.cryptoPuffiesContractWriteable !== null) {
      var tx = await wallet.cryptoPuffiesContractWriteable.mint(ethers.parseUnits("2.0", 18))
      await tx.wait()
    }
  }

  function updatePuffyToPreview(id: string) {
    return () => {
      var input: any = document.getElementById("search")
      console.log(input)
      if (input !== null) {
        input.value = id
        setPuffyToPreview(id)
      }
    }
  }

  function getPuffyToPreview(): string {
    return puffyToPreview === '' ? wallet.puffyTotalSupply : puffyToPreview
  }

  function getAttribute(id: string, attribute:string):string {
      var i = parseInt(id)
      if (attribute === "background") {
        return tokenData[i].background
      } else if (attribute === "color") {
        return tokenData[i].color
      } else if (attribute === "face") {
        return tokenData[i].face
      } else if (attribute === "hairstyle") {
        return tokenData[i].hairstyle
      } else if (attribute === "hat") {
        return tokenData[i].hat
      } else if (attribute === "tail") {
        return tokenData[i].tail
      } else if (attribute === "accessory") {
        return tokenData[i].accessory
      } else {
        return ""
      }
  }
  function onInputChange(e:any) {
    if (e.target.value < 0) {
      e.target.value = 0
    }
    if (e.target.value > 8887) {
      e.target.value = 8887
    }
    setPuffyToPreview(e.target.value)
  }

  return (
    <div className={styles.display}>
      {wallet.accounts.length > 0 &&
        <>
          <div>CryptoPuffies is a collection of 8888 randomly generated cute monster avatars living on the Avalanche network.</div>
          <div>Puffies Minted: {wallet.puffyTotalSupply}/8888 (ids: 0-8887)</div>
          {/*<button onClick={clickMint}>Mint Puffy (2 AVAX)</button>*/}
          <div>Next Minted Puffy Id: [<a onClick={updatePuffyToPreview(wallet.puffyTotalSupply)}>{wallet.puffyTotalSupply}</a>]</div>
          <hr></hr>
          <div>My Puffies ({wallet.puffyBalanceOfSelf}): 
            {wallet.puffyTokensOfSelf.map(id => (
              <span key={id}>{' '}[<a onClick={updatePuffyToPreview(id.toString())}>{id.toString()}</a>]</span>
            ))}
          </div>
          <hr></hr>
          Search: <input className={styles.search} name="search" id="search" type="number" min="0" max="8887" onChange={onInputChange}/>
          <div>
            <div className={styles.leftPanel}>
              <img className={styles.puffyImg} src={'images/'+getPuffyToPreview().toString()+'.jpg'}></img>
            </div>
            <div className={styles.rightPanel}>
              <div className={styles.puffyName}>Puffy #{getPuffyToPreview()}</div>
              <table className={styles.attributeTable}>
              <tr><th className={styles.attributeTableColumn}></th><th className={styles.attributeTableColumn}></th></tr>
              <tr><td>Background:</td><td>{getAttribute(getPuffyToPreview(), 'background')}</td></tr>
              <tr><td>Color:</td><td>{getAttribute(getPuffyToPreview(), 'color')}</td></tr>
              <tr><td>Face:</td><td>{getAttribute(getPuffyToPreview(), 'face')}</td></tr>
              <tr><td>Hairstyle:</td><td>{getAttribute(getPuffyToPreview(), 'hairstyle')}</td></tr>
              <tr><td>Hat:</td><td>{getAttribute(getPuffyToPreview(), 'hat')}</td></tr>
              <tr><td>Tail:</td><td>{getAttribute(getPuffyToPreview(), 'tail')}</td></tr>
              <tr><td>Accessory:</td><td>{getAttribute(getPuffyToPreview(), 'accessory')}</td></tr>
              </table>
              {/*
              <table className={styles.attributeTable}>
              <tr><th className={styles.attributeTableColumn}></th><th className={styles.attributeTableColumn}></th></tr>
              <tr><td>Health:</td><td>value</td></tr>
              <tr><td>Attack:</td><td>value</td></tr>
              <tr><td>Defense:</td><td>value</td></tr>
              <tr><td>Strength:</td><td>value</td></tr>
              </table>
              <table className={styles.attributeTable}>
              <tr><th className={styles.attributeTableColumn}></th><th className={styles.attributeTableColumn}></th></tr>
              <tr><td>Best Friend:</td><td>value</td></tr>
              <tr><td>Nemesis:</td><td>value</td></tr>
              </table>
              */}
            </div>
          </div>
        </>
      }
    </div>
  )
}