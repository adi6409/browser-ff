//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpChannel",
            base: Some("nsIIdentChannel"),
            methods: Ok(&[
                    /* [must_use] attribute ACString requestMethod; */
                    Method {
                        name: "GetRequestMethod",
                        params: &[Param { name: "aRequestMethod", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetRequestMethod",
                        params: &[Param { name: "aRequestMethod", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible,must_use] attribute nsIReferrerInfo referrerInfo; */
                    Method {
                        name: "GetReferrerInfo",
                        params: &[Param { name: "aReferrerInfo", ty: "*mut *const nsIReferrerInfo" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetReferrerInfo",
                        params: &[Param { name: "aReferrerInfo", ty: "*const nsIReferrerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use,noscript] void setReferrerInfoWithoutClone (in nsIReferrerInfo aReferrerInfo); */
                    Method {
                        name: "SetReferrerInfoWithoutClone",
                        params: &[Param { name: "aReferrerInfo", ty: "*const nsIReferrerInfo" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString protocolVersion; */
                    Method {
                        name: "GetProtocolVersion",
                        params: &[Param { name: "aProtocolVersion", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute uint64_t transferSize; */
                    Method {
                        name: "GetTransferSize",
                        params: &[Param { name: "aTransferSize", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute uint64_t requestSize; */
                    Method {
                        name: "GetRequestSize",
                        params: &[Param { name: "aRequestSize", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute uint64_t decodedBodySize; */
                    Method {
                        name: "GetDecodedBodySize",
                        params: &[Param { name: "aDecodedBodySize", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute uint64_t encodedBodySize; */
                    Method {
                        name: "GetEncodedBodySize",
                        params: &[Param { name: "aEncodedBodySize", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] ACString getRequestHeader (in ACString aHeader); */
                    Method {
                        name: "GetRequestHeader",
                        params: &[Param { name: "aHeader", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setRequestHeader (in ACString aHeader, in ACString aValue, in boolean aMerge); */
                    Method {
                        name: "SetRequestHeader",
                        params: &[Param { name: "aHeader", ty: "*const ::nsstring::nsACString" }, Param { name: "aValue", ty: "*const ::nsstring::nsACString" }, Param { name: "aMerge", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setNewReferrerInfo (in ACString aUrl, in nsIReferrerInfo_ReferrerPolicyIDL aPolicy, in boolean aSendReferrer); */
                    Method {
                        name: "SetNewReferrerInfo",
                        params: &[Param { name: "aUrl", ty: "*const ::nsstring::nsACString" }, Param { name: "aPolicy", ty: " u8" }, Param { name: "aSendReferrer", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setEmptyRequestHeader (in ACString aHeader); */
                    Method {
                        name: "SetEmptyRequestHeader",
                        params: &[Param { name: "aHeader", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void visitRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "VisitRequestHeaders",
                        params: &[Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void visitNonDefaultRequestHeaders (in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "VisitNonDefaultRequestHeaders",
                        params: &[Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] bool ShouldStripRequestBodyHeader (in ACString aMethod); */
                    Method {
                        name: "ShouldStripRequestBodyHeader",
                        params: &[Param { name: "aMethod", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute boolean allowPipelining; */
                    Method {
                        name: "GetAllowPipelining",
                        params: &[Param { name: "aAllowPipelining", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAllowPipelining",
                        params: &[Param { name: "aAllowPipelining", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute boolean allowSTS; */
                    Method {
                        name: "GetAllowSTS",
                        params: &[Param { name: "aAllowSTS", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetAllowSTS",
                        params: &[Param { name: "aAllowSTS", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute unsigned long redirectionLimit; */
                    Method {
                        name: "GetRedirectionLimit",
                        params: &[Param { name: "aRedirectionLimit", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetRedirectionLimit",
                        params: &[Param { name: "aRedirectionLimit", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute unsigned long responseStatus; */
                    Method {
                        name: "GetResponseStatus",
                        params: &[Param { name: "aResponseStatus", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute ACString responseStatusText; */
                    Method {
                        name: "GetResponseStatusText",
                        params: &[Param { name: "aResponseStatusText", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] readonly attribute boolean requestSucceeded; */
                    Method {
                        name: "GetRequestSucceeded",
                        params: &[Param { name: "aRequestSucceeded", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute boolean isMainDocumentChannel; */
                    Method {
                        name: "GetIsMainDocumentChannel",
                        params: &[Param { name: "aIsMainDocumentChannel", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetIsMainDocumentChannel",
                        params: &[Param { name: "aIsMainDocumentChannel", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] ACString getResponseHeader (in ACString header); */
                    Method {
                        name: "GetResponseHeader",
                        params: &[Param { name: "header", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void setResponseHeader (in ACString header, in ACString value, in boolean merge); */
                    Method {
                        name: "SetResponseHeader",
                        params: &[Param { name: "header", ty: "*const ::nsstring::nsACString" }, Param { name: "value", ty: "*const ::nsstring::nsACString" }, Param { name: "merge", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void visitResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "VisitResponseHeaders",
                        params: &[Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void getOriginalResponseHeader (in ACString aHeader, in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "GetOriginalResponseHeader",
                        params: &[Param { name: "aHeader", ty: "*const ::nsstring::nsACString" }, Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void visitOriginalResponseHeaders (in nsIHttpHeaderVisitor aVisitor); */
                    Method {
                        name: "VisitOriginalResponseHeaders",
                        params: &[Param { name: "aVisitor", ty: "*const nsIHttpHeaderVisitor" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean isNoStoreResponse (); */
                    Method {
                        name: "IsNoStoreResponse",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean isNoCacheResponse (); */
                    Method {
                        name: "IsNoCacheResponse",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] boolean isPrivateResponse (); */
                    Method {
                        name: "IsPrivateResponse",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void redirectTo (in nsIURI aTargetURI); */
                    Method {
                        name: "RedirectTo",
                        params: &[Param { name: "aTargetURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] void upgradeToSecure (); */
                    Method {
                        name: "UpgradeToSecure",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use,noscript] attribute uint64_t requestContextID; */
                    Method {
                        name: "GetRequestContextID",
                        params: &[Param { name: "aRequestContextID", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetRequestContextID",
                        params: &[Param { name: "aRequestContextID", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute uint64_t topLevelContentWindowId; */
                    Method {
                        name: "GetTopLevelContentWindowId",
                        params: &[Param { name: "aTopLevelContentWindowId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTopLevelContentWindowId",
                        params: &[Param { name: "aTopLevelContentWindowId", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute nsIHttpChannel_FlashPluginState flashPluginState; */
                    Method {
                        name: "GetFlashPluginState",
                        params: &[Param { name: "aFlashPluginState", ty: "*mut u8" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [must_use] attribute uint64_t topLevelOuterContentWindowId; */
                    Method {
                        name: "GetTopLevelOuterContentWindowId",
                        params: &[Param { name: "aTopLevelOuterContentWindowId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTopLevelOuterContentWindowId",
                        params: &[Param { name: "aTopLevelOuterContentWindowId", ty: "uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void logBlockedCORSRequest (in AString aMessage, in ACString aCategory); */
                    Method {
                        name: "LogBlockedCORSRequest",
                        params: &[Param { name: "aMessage", ty: "*const ::nsstring::nsAString" }, Param { name: "aCategory", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void logMimeTypeMismatch (in ACString aMessageName, in boolean aWarning, in AString aURL, in AString aContentType); */
                    Method {
                        name: "LogMimeTypeMismatch",
                        params: &[Param { name: "aMessageName", ty: "*const ::nsstring::nsACString" }, Param { name: "aWarning", ty: "bool" }, Param { name: "aURL", ty: "*const ::nsstring::nsAString" }, Param { name: "aContentType", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

