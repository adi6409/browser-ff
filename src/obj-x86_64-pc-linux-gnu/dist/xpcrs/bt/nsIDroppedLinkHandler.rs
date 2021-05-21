//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIDroppedLinkHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDroppedLinkItem",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString url; */
                    Method {
                        name: "GetUrl",
                        params: &[Param { name: "aUrl", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIDroppedLinkHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean canDropLink (in DragEvent aEvent, in boolean aAllowSameDocument); */
                    Method {
                        name: "CanDropLink",
                        params: &[Param { name: "aEvent", ty: "*const libc::c_void" }, Param { name: "aAllowSameDocument", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString dropLink (in DragEvent aEvent, out AString aName, [optional] in boolean aDisallowInherit); */
                    Method {
                        name: "DropLink",
                        params: &[Param { name: "aEvent", ty: "*const libc::c_void" }, Param { name: "aName", ty: "*mut ::nsstring::nsAString" }, Param { name: "aDisallowInherit", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsIDroppedLinkItem> dropLinks (in DragEvent aEvent, [optional] in boolean aDisallowInherit); */
                    Method {
                        name: "DropLinks",
                        params: &[Param { name: "aEvent", ty: "*const libc::c_void" }, Param { name: "aDisallowInherit", ty: "bool" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIDroppedLinkItem>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void validateURIsForDrop (in DragEvent aEvent, in Array<AString> aURIs, [optional] in boolean aDisallowInherit); */
                    Method {
                        name: "ValidateURIsForDrop",
                        params: &[Param { name: "aEvent", ty: "*const libc::c_void" }, Param { name: "aURIs", ty: "*const thin_vec::ThinVec<::nsstring::nsString>" }, Param { name: "aDisallowInherit", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsIDroppedLinkItem> queryLinks (in DataTransfer aDataTransfer); */
                    Method {
                        name: "QueryLinks",
                        params: &[Param { name: "aDataTransfer", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIDroppedLinkItem>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPrincipal getTriggeringPrincipal (in DragEvent aEvent); */
                    Method {
                        name: "GetTriggeringPrincipal",
                        params: &[Param { name: "aEvent", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIContentSecurityPolicy getCSP (in DragEvent aEvent); */
                    Method {
                        name: "GetCSP",
                        params: &[Param { name: "aEvent", ty: "*const libc::c_void" }, Param { name: "_retval", ty: "*mut *const nsIContentSecurityPolicy" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

