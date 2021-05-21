//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsIFaviconService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFaviconService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIURI getFaviconLinkForIcon (in nsIURI aFaviconURI); */
                    Method {
                        name: "GetFaviconLinkForIcon",
                        params: &[Param { name: "aFaviconURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void expireAllFavicons (); */
                    Method {
                        name: "ExpireAllFavicons",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDefaultIconURIPreferredSize (in unsigned short aDefaultSize); */
                    Method {
                        name: "SetDefaultIconURIPreferredSize",
                        params: &[Param { name: "aDefaultSize", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned short preferredSizeFromURI (in nsIURI aURI); */
                    Method {
                        name: "PreferredSizeFromURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI defaultFavicon; */
                    Method {
                        name: "GetDefaultFavicon",
                        params: &[Param { name: "aDefaultFavicon", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String defaultFaviconMimeType; */
                    Method {
                        name: "GetDefaultFaviconMimeType",
                        params: &[Param { name: "aDefaultFaviconMimeType", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIPlacesPendingOperation setAndFetchFaviconForPage (in nsIURI aPageURI, in nsIURI aFaviconURI, in boolean aForceReload, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback, [optional] in nsIPrincipal aLoadingPrincipal, [optional] in unsigned long long aRequestContextID); */
                    Method {
                        name: "SetAndFetchFaviconForPage",
                        params: &[Param { name: "aPageURI", ty: "*const nsIURI" }, Param { name: "aFaviconURI", ty: "*const nsIURI" }, Param { name: "aForceReload", ty: "bool" }, Param { name: "aFaviconLoadType", ty: "u32" }, Param { name: "aCallback", ty: "*const nsIFaviconDataCallback" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aRequestContextID", ty: "u64" }, Param { name: "_retval", ty: "*mut*const mozIPlacesPendingOperation" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void replaceFaviconData (in nsIURI aFaviconURI, in Array<octet> aData, in AUTF8String aMimeType, [optional] in PRTime aExpiration); */
                    Method {
                        name: "ReplaceFaviconData",
                        params: &[Param { name: "aFaviconURI", ty: "*const nsIURI" }, Param { name: "aData", ty: "*const thin_vec::ThinVec<u8>" }, Param { name: "aMimeType", ty: "*const ::nsstring::nsACString" }, Param { name: "aExpiration", ty: "PRTime" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void replaceFaviconDataFromDataURL (in nsIURI aFaviconURI, in AString aDataURL, [optional] in PRTime aExpiration, [optional] in nsIPrincipal aLoadingPrincipal); */
                    Method {
                        name: "ReplaceFaviconDataFromDataURL",
                        params: &[Param { name: "aFaviconURI", ty: "*const nsIURI" }, Param { name: "aDataURL", ty: "*const ::nsstring::nsAString" }, Param { name: "aExpiration", ty: "PRTime" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getFaviconURLForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
                    Method {
                        name: "GetFaviconURLForPage",
                        params: &[Param { name: "aPageURI", ty: "*const nsIURI" }, Param { name: "aCallback", ty: "*const nsIFaviconDataCallback" }, Param { name: "aPreferredWidth", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getFaviconDataForPage (in nsIURI aPageURI, in nsIFaviconDataCallback aCallback, [optional] in unsigned short aPreferredWidth); */
                    Method {
                        name: "GetFaviconDataForPage",
                        params: &[Param { name: "aPageURI", ty: "*const nsIURI" }, Param { name: "aCallback", ty: "*const nsIFaviconDataCallback" }, Param { name: "aPreferredWidth", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void copyFavicons (in nsIURI aFromPageURI, in nsIURI aToPageURI, in unsigned long aFaviconLoadType, [optional] in nsIFaviconDataCallback aCallback); */
                    Method {
                        name: "CopyFavicons",
                        params: &[Param { name: "aFromPageURI", ty: "*const nsIURI" }, Param { name: "aToPageURI", ty: "*const nsIURI" }, Param { name: "aFaviconLoadType", ty: "u32" }, Param { name: "aCallback", ty: "*const nsIFaviconDataCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIFaviconDataCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onComplete (in nsIURI aFaviconURI, in unsigned long aDataLen, [array, size_is (aDataLen), const] in octet aData, in AUTF8String aMimeType, in unsigned short aWidth); */
                    Method {
                        name: "OnComplete",
                        params: &[Param { name: "aFaviconURI", ty: "*const nsIURI" }, Param { name: "aDataLen", ty: "u32" }, Param { name: "aData", ty: "*const u8" }, Param { name: "aMimeType", ty: "*const ::nsstring::nsACString" }, Param { name: "aWidth", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

