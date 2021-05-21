//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsITransfer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransfer",
            base: Some("nsIWebProgressListener2"),
            methods: Ok(&[
                    /* void init (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate, in long aDownloadClassification); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aSource", ty: "*const nsIURI" }, Param { name: "aTarget", ty: "*const nsIURI" }, Param { name: "aDisplayName", ty: "*const ::nsstring::nsAString" }, Param { name: "aMIMEInfo", ty: "*const nsIMIMEInfo" }, Param { name: "startTime", ty: "PRTime" }, Param { name: "aTempFile", ty: "*const nsIFile" }, Param { name: "aCancelable", ty: "*const nsICancelable" }, Param { name: "aIsPrivate", ty: "bool" }, Param { name: "aDownloadClassification", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void initWithBrowsingContext (in nsIURI aSource, in nsIURI aTarget, in AString aDisplayName, in nsIMIMEInfo aMIMEInfo, in PRTime startTime, in nsIFile aTempFile, in nsICancelable aCancelable, in boolean aIsPrivate, in long aDownloadClassification, in BrowsingContext aBrowsingContext, in boolean aHandleInternally); */
                    Method {
                        name: "InitWithBrowsingContext",
                        params: &[Param { name: "aSource", ty: "*const nsIURI" }, Param { name: "aTarget", ty: "*const nsIURI" }, Param { name: "aDisplayName", ty: "*const ::nsstring::nsAString" }, Param { name: "aMIMEInfo", ty: "*const nsIMIMEInfo" }, Param { name: "startTime", ty: "PRTime" }, Param { name: "aTempFile", ty: "*const nsIFile" }, Param { name: "aCancelable", ty: "*const nsICancelable" }, Param { name: "aIsPrivate", ty: "bool" }, Param { name: "aDownloadClassification", ty: "i32" }, Param { name: "aBrowsingContext", ty: "*const libc::c_void" }, Param { name: "aHandleInternally", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSha256Hash (in ACString aHash); */
                    Method {
                        name: "SetSha256Hash",
                        params: &[Param { name: "aHash", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSignatureInfo (in Array<Array<Array<uint8_t>>> aSignatureInfo); */
                    Method {
                        name: "SetSignatureInfo",
                        params: &[Param { name: "aSignatureInfo", ty: "*const thin_vec::ThinVec<thin_vec::ThinVec<thin_vec::ThinVec<uint8_t>>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setRedirects (in nsIArray aRedirects); */
                    Method {
                        name: "SetRedirects",
                        params: &[Param { name: "aRedirects", ty: "*const nsIArray" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

