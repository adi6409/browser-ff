/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIHTTPHeaderListener.idl
 */

#ifndef __gen_nsIHTTPHeaderListener_h__
#define __gen_nsIHTTPHeaderListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIHTTPHeaderListener */
#define NS_IHTTPHEADERLISTENER_IID_STR "ea51e0b8-871c-4b85-92da-6f400394c5ec"

#define NS_IHTTPHEADERLISTENER_IID \
  {0xea51e0b8, 0x871c, 0x4b85, \
    { 0x92, 0xda, 0x6f, 0x40, 0x03, 0x94, 0xc5, 0xec }}

class NS_NO_VTABLE nsIHTTPHeaderListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPHEADERLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHTTPHeaderListener;

  /* void newResponseHeader (in string headerName, in string headerValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NewResponseHeader(const char * headerName, const char * headerValue) = 0;

  /* void statusLine (in string line); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StatusLine(const char * line) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHTTPHeaderListener, NS_IHTTPHEADERLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPHEADERLISTENER \
  NS_IMETHOD NewResponseHeader(const char * headerName, const char * headerValue) override; \
  NS_IMETHOD StatusLine(const char * line) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPHEADERLISTENER \
  nsresult NewResponseHeader(const char * headerName, const char * headerValue); \
  nsresult StatusLine(const char * line); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPHEADERLISTENER(_to) \
  NS_IMETHOD NewResponseHeader(const char * headerName, const char * headerValue) override { return _to NewResponseHeader(headerName, headerValue); } \
  NS_IMETHOD StatusLine(const char * line) override { return _to StatusLine(line); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPHEADERLISTENER(_to) \
  NS_IMETHOD NewResponseHeader(const char * headerName, const char * headerValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewResponseHeader(headerName, headerValue); } \
  NS_IMETHOD StatusLine(const char * line) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StatusLine(line); } 


#endif /* __gen_nsIHTTPHeaderListener_h__ */
