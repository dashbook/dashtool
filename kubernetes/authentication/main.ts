import * as oauth from 'https://deno.land/x/oauth4webapi@v2.3.0/mod.ts'

const issuerUrl = Deno.env.get("ISSUER");
const clientId = Deno.env.get("CLIENT_ID");

const issuer = new URL(issuerUrl)
const as = await oauth
    .discoveryRequest(issuer)
    .then((response) => oauth.processDiscoveryResponse(issuer, response))

const client: oauth.Client = {
    client_id: clientId,
    token_endpoint_auth_method: 'none',
}

let device_code: string
let interval: number
let verification_uri_complete: string | undefined

const parameters = new URLSearchParams()
parameters.set('scope', 'offline_access openid')

{
    const response = await oauth.deviceAuthorizationRequest(as, client, parameters)
    let challenges: oauth.WWWAuthenticateChallenge[] | undefined
    if ((challenges = oauth.parseWwwAuthenticateChallenges(response))) {
        for (const challenge of challenges) {
            console.log('challenge', challenge)
        }
        throw new Error() // Handle www-authenticate challenges as needed
    }

    const result = await oauth.processDeviceAuthorizationResponse(as, client, response)
    if (oauth.isOAuth2Error(result)) {
        console.log('error', result)
        throw new Error() // Handle OAuth 2.0 response body error
    }

    ; ({ device_code, verification_uri_complete, interval = 5 } = result)
}

// user gets shown the verification_uri and user_code, or scans a qr code formed from verification_uri_complete as input
// user starts authenticating on his other device
console.log(`Please login here: ${verification_uri_complete}`)

function wait() {
    return new Promise((resolve) => {
        setTimeout(resolve, interval * 1000)
    })
}

let success: oauth.TokenEndpointResponse | undefined = undefined

while (success === undefined) {
    await wait()
    const response = await oauth.deviceCodeGrantRequest(as, client, device_code, parameters)
    let challenges: oauth.WWWAuthenticateChallenge[] | undefined
    if ((challenges = oauth.parseWwwAuthenticateChallenges(response))) {
        for (const challenge of challenges) {
            console.log('challenge', challenge)
        }
        throw new Error() // Handle www-authenticate challenges as needed
    }

    const result = await oauth.processDeviceCodeResponse(as, client, response)

    if (oauth.isOAuth2Error(result)) {
        // response is oauth style error object
        switch (result.error) {
            case 'slow_down':
                interval += 5
            case 'authorization_pending':
                continue
            default:
                console.log('error', result)
                throw new Error() // Handle OAuth 2.0 response body error
        }
    } else {
        success = result
    }
}

if (success.refresh_token) {
    let encoder = new TextEncoder();
    let data = encoder.encode(success.refresh_token);
    Deno.writeFile("/tmp/token/refresh.jwt", data);
} else {
    console.log("No refresh token")
    throw new Error()
}