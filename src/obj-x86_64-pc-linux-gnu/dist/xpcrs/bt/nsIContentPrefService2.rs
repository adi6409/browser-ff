//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentPrefService2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentPrefObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onContentPrefSet (in AString aGroup, in AString aName, in nsIVariant aValue, [optional] in boolean aIsPrivate); */
                    Method {
                        name: "OnContentPrefSet",
                        params: &[Param { name: "aGroup", ty: "*const ::nsstring::nsAString" }, Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "aValue", ty: "*const nsIVariant" }, Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onContentPrefRemoved (in AString aGroup, in AString aName, [optional] in boolean aIsPrivate); */
                    Method {
                        name: "OnContentPrefRemoved",
                        params: &[Param { name: "aGroup", ty: "*const ::nsstring::nsAString" }, Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIContentPrefService2",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void getByName (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "GetByName",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getByDomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "GetByDomainAndName",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "GetBySubdomainAndName",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getGlobal (in AString name, in nsILoadContext context, in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "GetGlobal",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIContentPref getCachedByDomainAndName (in AString domain, in AString name, in nsILoadContext context); */
                    Method {
                        name: "GetCachedByDomainAndName",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "_retval", ty: "*mut*const nsIContentPref" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Array<nsIContentPref> getCachedBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context); */
                    Method {
                        name: "GetCachedBySubdomainAndName",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "_retval", ty: "*mut thin_vec::ThinVec<RefPtr<nsIContentPref>>" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIContentPref getCachedGlobal (in AString name, in nsILoadContext context); */
                    Method {
                        name: "GetCachedGlobal",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "_retval", ty: "*mut*const nsIContentPref" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void set (in AString domain, in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "Set",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const nsIVariant" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setGlobal (in AString name, in nsIVariant value, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "SetGlobal",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const nsIVariant" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeByDomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveByDomainAndName",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeBySubdomainAndName (in AString domain, in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveBySubdomainAndName",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeGlobal (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveGlobal",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeByDomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveByDomain",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeBySubdomain (in AString domain, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveBySubdomain",
                        params: &[Param { name: "domain", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeByName (in AString name, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveByName",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeAllDomains (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveAllDomains",
                        params: &[Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeAllDomainsSince (in unsigned long long since, in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveAllDomainsSince",
                        params: &[Param { name: "since", ty: "u64" }, Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeAllGlobals (in nsILoadContext context, [optional] in nsIContentPrefCallback2 callback); */
                    Method {
                        name: "RemoveAllGlobals",
                        params: &[Param { name: "context", ty: "*const nsILoadContext" }, Param { name: "callback", ty: "*const nsIContentPrefCallback2" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addObserverForName (in AString name, in nsIContentPrefObserver observer); */
                    Method {
                        name: "AddObserverForName",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "observer", ty: "*const nsIContentPrefObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeObserverForName (in AString name, in nsIContentPrefObserver observer); */
                    Method {
                        name: "RemoveObserverForName",
                        params: &[Param { name: "name", ty: "*const ::nsstring::nsAString" }, Param { name: "observer", ty: "*const nsIContentPrefObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString extractDomain (in AString str); */
                    Method {
                        name: "ExtractDomain",
                        params: &[Param { name: "str", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIContentPrefCallback2",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleResult (in nsIContentPref pref); */
                    Method {
                        name: "HandleResult",
                        params: &[Param { name: "pref", ty: "*const nsIContentPref" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleError (in nsresult error); */
                    Method {
                        name: "HandleError",
                        params: &[Param { name: "error", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleCompletion (in unsigned short reason); */
                    Method {
                        name: "HandleCompletion",
                        params: &[Param { name: "reason", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIContentPref",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString domain; */
                    Method {
                        name: "GetDomain",
                        params: &[Param { name: "aDomain", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIVariant value; */
                    Method {
                        name: "GetValue",
                        params: &[Param { name: "aValue", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

