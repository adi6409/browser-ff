//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITLSServerSocket.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITLSServerSocket",
            base: Some("nsIServerSocket"),
            methods: Ok(&[
                    /* attribute nsIX509Cert serverCert; */
                    Method {
                        name: "GetServerCert",
                        params: &[Param { name: "aServerCert", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetServerCert",
                        params: &[Param { name: "aServerCert", ty: "*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSessionTickets (in boolean aSessionTickets); */
                    Method {
                        name: "SetSessionTickets",
                        params: &[Param { name: "aSessionTickets", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setRequestClientCertificate (in unsigned long aRequestClientCert); */
                    Method {
                        name: "SetRequestClientCertificate",
                        params: &[Param { name: "aRequestClientCert", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setVersionRange (in unsigned short aMinVersion, in unsigned short aMaxVersion); */
                    Method {
                        name: "SetVersionRange",
                        params: &[Param { name: "aMinVersion", ty: "u16" }, Param { name: "aMaxVersion", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITLSClientStatus",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute nsIX509Cert peerCert; */
                    Method {
                        name: "GetPeerCert",
                        params: &[Param { name: "aPeerCert", ty: "*mut*const nsIX509Cert" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute short tlsVersionUsed; */
                    Method {
                        name: "GetTlsVersionUsed",
                        params: &[Param { name: "aTlsVersionUsed", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString cipherName; */
                    Method {
                        name: "GetCipherName",
                        params: &[Param { name: "aCipherName", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long keyLength; */
                    Method {
                        name: "GetKeyLength",
                        params: &[Param { name: "aKeyLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long macLength; */
                    Method {
                        name: "GetMacLength",
                        params: &[Param { name: "aMacLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITLSServerConnectionInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setSecurityObserver (in nsITLSServerSecurityObserver observer); */
                    Method {
                        name: "SetSecurityObserver",
                        params: &[Param { name: "observer", ty: "*const nsITLSServerSecurityObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsITLSServerSocket serverSocket; */
                    Method {
                        name: "GetServerSocket",
                        params: &[Param { name: "aServerSocket", ty: "*mut *const nsITLSServerSocket" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsITLSClientStatus status; */
                    Method {
                        name: "GetStatus",
                        params: &[Param { name: "aStatus", ty: "*mut *const nsITLSClientStatus" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsITLSServerSecurityObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onHandshakeDone (in nsITLSServerSocket aServer, in nsITLSClientStatus aStatus); */
                    Method {
                        name: "OnHandshakeDone",
                        params: &[Param { name: "aServer", ty: "*const nsITLSServerSocket" }, Param { name: "aStatus", ty: "*const nsITLSClientStatus" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

