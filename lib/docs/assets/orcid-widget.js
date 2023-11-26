(function(global) {

	var clientId;
	var onSuccess;
	var onFail;

	const sandboxIssuer = "https://sandbox.orcid.org";
	const sandboxUrl = "https://sandbox.orcid.org/oauth/authorize";
	const sandboxKey = {"kty":"RSA","e":"AQAB","use":"sig","kid":"sandbox-orcid-org-3hpgosl3b6lapenh1ewsgdob3fawepoj","n":"pl-jp-kTAGf6BZUrWIYUJTvqqMVd4iAnoLS6vve-KNV0q8TxKvMre7oi9IulDcqTuJ1alHrZAIVlgrgFn88MKirZuTqHG6LCtEsr7qGD9XyVcz64oXrb9vx4FO9tLNQxvdnIWCIwyPAYWtPMHMSSD5oEVUtVL_5IaxfCJvU-FchdHiwfxvXMWmA-i3mcEEe9zggag2vUPPIqUwbPVUFNj2hE7UsZbasuIToEMFRZqSB6juc9zv6PEUueQ5hAJCEylTkzMwyBMibrt04TmtZk2w9DfKJR91555s2ZMstX4G_su1_FqQ6p9vgcuLQ6tCtrW77tta-Rw7McF_tyPmvnhQ"};
	const liveIssuer = "https://orcid.org";
	const liveUrl = "https://orcid.org/oauth/authorize";
	const liveKey = {"kty":"RSA","e":"AQAB","use":"sig","kid":"production-orcid-org-7hdmdswarosg3gjujo8agwtazgkp1ojs","n":"jxTIntA7YvdfnYkLSN4wk__E2zf_wbb0SV_HLHFvh6a9ENVRD1_rHK0EijlBzikb-1rgDQihJETcgBLsMoZVQqGj8fDUUuxnVHsuGav_bf41PA7E_58HXKPrB2C0cON41f7K3o9TStKpVJOSXBrRWURmNQ64qnSSryn1nCxMzXpaw7VUo409ohybbvN6ngxVy4QR2NCC7Fr0QVdtapxD7zdlwx6lEwGemuqs_oG5oDtrRuRgeOHmRps2R6gG5oc-JqVMrVRv6F9h4ja3UgxCDBQjOVT1BFPWmMHnHCsVYLqbbXkZUfvP2sO1dJiYd_zrQhi-FtNth9qrLLv3gkgtwQ"};
	var issuer;
	var authUrl;
	var key;
	var returnUrl;
	var pubKey;
	var buttonText;

	var signedInOrcid;
	var signedInIdToken;

	function init(config){
		clientId = config.clientId;
		onSuccess = config.onSuccess;
		onFail = config.onFail;
		returnUrl = config.returnUrl;
		buttonText = config.buttonText;

		if (config.mode === 'live'){
			issuer = liveIssuer;
			authUrl = liveUrl;
			key = liveKey;
		}else{
			issuer = sandboxIssuer;
			authUrl = sandboxUrl;
			key = sandboxKey;
		}
		pubKey = KEYUTIL.getKey(key);

		if (config.auto){
			//check for response - if exists process it
			if (getFragmentParameterByName("id_token")){
				handleResponse();
			}

			//if we don't have a signed in user, show sign in button
			if (!signedInOrcid){
				createButton('orcidWidget');
			}
		}

	}

	function createButton(elementId){
    var newLink = document.createElement("a");
    newLink.id = "orcidLink";
    newLink.href = buildReturnUrl();
    newLink.textContent = buttonText;
    var parentElement = document.getElementById(elementId);
    parentElement.appendChild(newLink);
	}

	function getFragmentParameterByName(name) {
	    name = name.replace(/[\[]/, "\\[").replace(/[\]]/, "\\]");
	    var regex = new RegExp("[\\#&]" + name + "=([^&#]*)"),
	        results = regex.exec(window.location.hash);
	    return results === null ? "" : decodeURIComponent(results[1].replace(/\+/g, " "));
	}

	function checkSig(idToken){
	    return KJUR.jws.JWS.verifyJWT(idToken, pubKey, {
	    	alg: ['RS256'], iss: [issuer] , aud:clientId,gracePeriod: 15*60 //15 mins skew allowed
	  	});
	}

	function buildReturnUrl(nonce){
		var url = authUrl+"?response_type=token&redirect_uri="+returnUrl+"&client_id="+clientId+"&scope=openid";
		if (nonce)
			url += "&nonce="+nonce;
		return url;
	}

	function handleResponse(){
		var id_token = getFragmentParameterByName("id_token");
		if(id_token){
			if (checkSig(id_token)){
          signedInIdToken = JSON.parse(KJUR.jws.JWS.parse(id_token).payloadPP);
          signedInOrcid = signedInIdToken.sub;
          onSuccess(signedInIdToken);
		    }else{
          signedInIdToken = null;
          signedInOrcid = null;
          onFail();
		    }
		}
	}

	global.ORCID = {
	 	init:init,
	 	signedInOrcid:signedInOrcid,
	 	signedInIdToken:signedInIdToken,
	 	handleResponse:handleResponse,
	 	buildReturnUrl:buildReturnUrl
	}

})(this);
