//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPermissionManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPermissionManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* Array<nsIPermission> getAllForPrincipal (in nsIPrincipal principal); */
                    Method {
                        name: "GetAllForPrincipal",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIPermission>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsIPermission> getAllWithTypePrefix (in ACString prefix); */
                    Method {
                        name: "GetAllWithTypePrefix",
                        params: &[Param { name: "prefix", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIPermission>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsIPermission> getAllByTypeSince (in ACString type, in int64_t since); */
                    Method {
                        name: "GetAllByTypeSince",
                        params: &[Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "since", ty: "int64_t" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIPermission>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addFromPrincipal (in nsIPrincipal principal, in ACString type, in uint32_t permission, [optional] in uint32_t expireType, [optional] in int64_t expireTime); */
                    Method {
                        name: "AddFromPrincipal",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "permission", ty: "uint32_t" }, Param { name: "expireType", ty: "uint32_t" }, Param { name: "expireTime", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeFromPrincipal (in nsIPrincipal principal, in ACString type); */
                    Method {
                        name: "RemoveFromPrincipal",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "type_", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removePermission (in nsIPermission perm); */
                    Method {
                        name: "RemovePermission",
                        params: &[Param { name: "perm", ty: "*const nsIPermission" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeAll (); */
                    Method {
                        name: "RemoveAll",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeAllSince (in int64_t since); */
                    Method {
                        name: "RemoveAllSince",
                        params: &[Param { name: "since", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeByType (in ACString type); */
                    Method {
                        name: "RemoveByType",
                        params: &[Param { name: "type_", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeByTypeSince (in ACString type, in int64_t since); */
                    Method {
                        name: "RemoveByTypeSince",
                        params: &[Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "since", ty: "int64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* uint32_t testPermissionFromPrincipal (in nsIPrincipal principal, in ACString type); */
                    Method {
                        name: "TestPermissionFromPrincipal",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* uint32_t testExactPermissionFromPrincipal (in nsIPrincipal principal, in ACString type); */
                    Method {
                        name: "TestExactPermissionFromPrincipal",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* uint32_t testExactPermanentPermission (in nsIPrincipal principal, in ACString type); */
                    Method {
                        name: "TestExactPermanentPermission",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIPermission getPermissionObject (in nsIPrincipal principal, in ACString type, in boolean exactHost); */
                    Method {
                        name: "GetPermissionObject",
                        params: &[Param { name: "principal", ty: "*const nsIPrincipal" }, Param { name: "type_", ty: "*const ::nsstring::nsACString" }, Param { name: "exactHost", ty: "bool" }, Param { name: "_retval", ty: "*mut*const nsIPermission" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Array<nsIPermission> all; */
                    Method {
                        name: "GetAll",
                        params: &[Param { name: "aAll", ty: "*mut thin_vec::ThinVec<RefPtr<nsIPermission>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removePermissionsWithAttributes (in AString patternAsJSON); */
                    Method {
                        name: "RemovePermissionsWithAttributes",
                        params: &[Param { name: "patternAsJSON", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void broadcastPermissionsForPrincipalToAllContentProcesses (in nsIPrincipal aPrincipal); */
                    Method {
                        name: "BroadcastPermissionsForPrincipalToAllContentProcesses",
                        params: &[Param { name: "aPrincipal", ty: "*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

