/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIDirIndexListener.idl
 */

#ifndef __gen_nsIDirIndexListener_h__
#define __gen_nsIDirIndexListener_h__


#ifndef __gen_nsIStreamListener_h__
#include "nsIStreamListener.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDirIndex; /* forward declaration */


/* starting interface:    nsIDirIndexListener */
#define NS_IDIRINDEXLISTENER_IID_STR "fae4e9a8-1dd1-11b2-b53c-8f3aa1bbf8f5"

#define NS_IDIRINDEXLISTENER_IID \
  {0xfae4e9a8, 0x1dd1, 0x11b2, \
    { 0xb5, 0x3c, 0x8f, 0x3a, 0xa1, 0xbb, 0xf8, 0xf5 }}

class NS_NO_VTABLE nsIDirIndexListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDIRINDEXLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDirIndexListener;

  /* void onIndexAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in nsIDirIndex aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnIndexAvailable(nsIRequest *aRequest, nsISupports *aCtxt, nsIDirIndex *aIndex) = 0;

  /* void onInformationAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in AString aInfo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnInformationAvailable(nsIRequest *aRequest, nsISupports *aCtxt, const nsAString& aInfo) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDirIndexListener, NS_IDIRINDEXLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDIRINDEXLISTENER \
  NS_IMETHOD OnIndexAvailable(nsIRequest *aRequest, nsISupports *aCtxt, nsIDirIndex *aIndex) override; \
  NS_IMETHOD OnInformationAvailable(nsIRequest *aRequest, nsISupports *aCtxt, const nsAString& aInfo) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDIRINDEXLISTENER \
  nsresult OnIndexAvailable(nsIRequest *aRequest, nsISupports *aCtxt, nsIDirIndex *aIndex); \
  nsresult OnInformationAvailable(nsIRequest *aRequest, nsISupports *aCtxt, const nsAString& aInfo); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDIRINDEXLISTENER(_to) \
  NS_IMETHOD OnIndexAvailable(nsIRequest *aRequest, nsISupports *aCtxt, nsIDirIndex *aIndex) override { return _to OnIndexAvailable(aRequest, aCtxt, aIndex); } \
  NS_IMETHOD OnInformationAvailable(nsIRequest *aRequest, nsISupports *aCtxt, const nsAString& aInfo) override { return _to OnInformationAvailable(aRequest, aCtxt, aInfo); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDIRINDEXLISTENER(_to) \
  NS_IMETHOD OnIndexAvailable(nsIRequest *aRequest, nsISupports *aCtxt, nsIDirIndex *aIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnIndexAvailable(aRequest, aCtxt, aIndex); } \
  NS_IMETHOD OnInformationAvailable(nsIRequest *aRequest, nsISupports *aCtxt, const nsAString& aInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnInformationAvailable(aRequest, aCtxt, aInfo); } 


/* starting interface:    nsIDirIndexParser */
#define NS_IDIRINDEXPARSER_IID_STR "38e3066c-1dd2-11b2-9b59-8be515c1ee3f"

#define NS_IDIRINDEXPARSER_IID \
  {0x38e3066c, 0x1dd2, 0x11b2, \
    { 0x9b, 0x59, 0x8b, 0xe5, 0x15, 0xc1, 0xee, 0x3f }}

class NS_NO_VTABLE nsIDirIndexParser : public nsIStreamListener {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDIRINDEXPARSER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDirIndexParser;

  /* attribute nsIDirIndexListener listener; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListener(nsIDirIndexListener **aListener) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetListener(nsIDirIndexListener *aListener) = 0;

  /* readonly attribute string comment; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetComment(char * *aComment) = 0;

  /* attribute string encoding; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEncoding(char * *aEncoding) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEncoding(const char * aEncoding) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDirIndexParser, NS_IDIRINDEXPARSER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDIRINDEXPARSER \
  NS_IMETHOD GetListener(nsIDirIndexListener **aListener) override; \
  NS_IMETHOD SetListener(nsIDirIndexListener *aListener) override; \
  NS_IMETHOD GetComment(char * *aComment) override; \
  NS_IMETHOD GetEncoding(char * *aEncoding) override; \
  NS_IMETHOD SetEncoding(const char * aEncoding) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDIRINDEXPARSER \
  nsresult GetListener(nsIDirIndexListener **aListener); \
  nsresult SetListener(nsIDirIndexListener *aListener); \
  nsresult GetComment(char * *aComment); \
  nsresult GetEncoding(char * *aEncoding); \
  nsresult SetEncoding(const char * aEncoding); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDIRINDEXPARSER(_to) \
  NS_IMETHOD GetListener(nsIDirIndexListener **aListener) override { return _to GetListener(aListener); } \
  NS_IMETHOD SetListener(nsIDirIndexListener *aListener) override { return _to SetListener(aListener); } \
  NS_IMETHOD GetComment(char * *aComment) override { return _to GetComment(aComment); } \
  NS_IMETHOD GetEncoding(char * *aEncoding) override { return _to GetEncoding(aEncoding); } \
  NS_IMETHOD SetEncoding(const char * aEncoding) override { return _to SetEncoding(aEncoding); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDIRINDEXPARSER(_to) \
  NS_IMETHOD GetListener(nsIDirIndexListener **aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListener(aListener); } \
  NS_IMETHOD SetListener(nsIDirIndexListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetListener(aListener); } \
  NS_IMETHOD GetComment(char * *aComment) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetComment(aComment); } \
  NS_IMETHOD GetEncoding(char * *aEncoding) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEncoding(aEncoding); } \
  NS_IMETHOD SetEncoding(const char * aEncoding) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEncoding(aEncoding); } 


#endif /* __gen_nsIDirIndexListener_h__ */
