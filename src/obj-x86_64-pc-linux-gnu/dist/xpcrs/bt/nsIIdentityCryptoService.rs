//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/crypto/component/nsIIdentityCryptoService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIdentityCryptoService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void generateKeyPair (in AUTF8String algorithm, in nsIIdentityKeyGenCallback callback); */
                    Method {
                        name: "GenerateKeyPair",
                        params: &[Param { name: "algorithm", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const nsIIdentityKeyGenCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString base64UrlEncode (in AUTF8String toEncode); */
                    Method {
                        name: "Base64UrlEncode",
                        params: &[Param { name: "toEncode", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIIdentityKeyPair",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AUTF8String keyType; */
                    Method {
                        name: "GetKeyType",
                        params: &[Param { name: "aKeyType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String hexRSAPublicKeyExponent; */
                    Method {
                        name: "GetHexRSAPublicKeyExponent",
                        params: &[Param { name: "aHexRSAPublicKeyExponent", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String hexRSAPublicKeyModulus; */
                    Method {
                        name: "GetHexRSAPublicKeyModulus",
                        params: &[Param { name: "aHexRSAPublicKeyModulus", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String hexDSAPrime; */
                    Method {
                        name: "GetHexDSAPrime",
                        params: &[Param { name: "aHexDSAPrime", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String hexDSASubPrime; */
                    Method {
                        name: "GetHexDSASubPrime",
                        params: &[Param { name: "aHexDSASubPrime", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String hexDSAGenerator; */
                    Method {
                        name: "GetHexDSAGenerator",
                        params: &[Param { name: "aHexDSAGenerator", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String hexDSAPublicValue; */
                    Method {
                        name: "GetHexDSAPublicValue",
                        params: &[Param { name: "aHexDSAPublicValue", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void sign (in AUTF8String aText, in nsIIdentitySignCallback callback); */
                    Method {
                        name: "Sign",
                        params: &[Param { name: "aText", ty: "*const ::nsstring::nsACString" }, Param { name: "callback", ty: "*const nsIIdentitySignCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIIdentityKeyGenCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void generateKeyPairFinished (in nsresult rv, in nsIIdentityKeyPair keyPair); */
                    Method {
                        name: "GenerateKeyPairFinished",
                        params: &[Param { name: "rv", ty: "::nserror::nsresult" }, Param { name: "keyPair", ty: "*const nsIIdentityKeyPair" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIIdentitySignCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void signFinished (in nsresult rv, in ACString base64urlSignature); */
                    Method {
                        name: "SignFinished",
                        params: &[Param { name: "rv", ty: "::nserror::nsresult" }, Param { name: "base64urlSignature", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

