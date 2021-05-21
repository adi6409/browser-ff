/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIMIMEInputStream.idl
 */

#ifndef __gen_nsIMIMEInputStream_h__
#define __gen_nsIMIMEInputStream_h__


#ifndef __gen_nsIHttpHeaderVisitor_h__
#include "nsIHttpHeaderVisitor.h"
#endif

#ifndef __gen_nsIInputStream_h__
#include "nsIInputStream.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIMIMEInputStream */
#define NS_IMIMEINPUTSTREAM_IID_STR "dcbce63c-1dd1-11b2-b94d-91f6d49a3161"

#define NS_IMIMEINPUTSTREAM_IID \
  {0xdcbce63c, 0x1dd1, 0x11b2, \
    { 0xb9, 0x4d, 0x91, 0xf6, 0xd4, 0x9a, 0x31, 0x61 }}

class NS_NO_VTABLE nsIMIMEInputStream : public nsIInputStream {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMIMEINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMIMEInputStream;

  /* void addHeader (in string name, in string value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddHeader(const char * name, const char * value) = 0;

  /* void visitHeaders (in nsIHttpHeaderVisitor visitor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD VisitHeaders(nsIHttpHeaderVisitor *visitor) = 0;

  /* void setData (in nsIInputStream stream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetData(nsIInputStream *stream) = 0;

  /* readonly attribute nsIInputStream data; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetData(nsIInputStream **aData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMIMEInputStream, NS_IMIMEINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMIMEINPUTSTREAM \
  NS_IMETHOD AddHeader(const char * name, const char * value) override; \
  NS_IMETHOD VisitHeaders(nsIHttpHeaderVisitor *visitor) override; \
  NS_IMETHOD SetData(nsIInputStream *stream) override; \
  NS_IMETHOD GetData(nsIInputStream **aData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMIMEINPUTSTREAM \
  nsresult AddHeader(const char * name, const char * value); \
  nsresult VisitHeaders(nsIHttpHeaderVisitor *visitor); \
  nsresult SetData(nsIInputStream *stream); \
  nsresult GetData(nsIInputStream **aData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMIMEINPUTSTREAM(_to) \
  NS_IMETHOD AddHeader(const char * name, const char * value) override { return _to AddHeader(name, value); } \
  NS_IMETHOD VisitHeaders(nsIHttpHeaderVisitor *visitor) override { return _to VisitHeaders(visitor); } \
  NS_IMETHOD SetData(nsIInputStream *stream) override { return _to SetData(stream); } \
  NS_IMETHOD GetData(nsIInputStream **aData) override { return _to GetData(aData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMIMEINPUTSTREAM(_to) \
  NS_IMETHOD AddHeader(const char * name, const char * value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddHeader(name, value); } \
  NS_IMETHOD VisitHeaders(nsIHttpHeaderVisitor *visitor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitHeaders(visitor); } \
  NS_IMETHOD SetData(nsIInputStream *stream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetData(stream); } \
  NS_IMETHOD GetData(nsIInputStream **aData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetData(aData); } 


#endif /* __gen_nsIMIMEInputStream_h__ */
