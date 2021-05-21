/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpHeaderVisitor.idl
 */

#ifndef __gen_nsIHttpHeaderVisitor_h__
#define __gen_nsIHttpHeaderVisitor_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIHttpHeaderVisitor */
#define NS_IHTTPHEADERVISITOR_IID_STR "35412859-b9d9-423c-8866-2d4559fdd2be"

#define NS_IHTTPHEADERVISITOR_IID \
  {0x35412859, 0xb9d9, 0x423c, \
    { 0x88, 0x66, 0x2d, 0x45, 0x59, 0xfd, 0xd2, 0xbe }}

class NS_NO_VTABLE nsIHttpHeaderVisitor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPHEADERVISITOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpHeaderVisitor;

  /* [must_use] void visitHeader (in ACString aHeader, in ACString aValue); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD VisitHeader(const nsACString& aHeader, const nsACString& aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpHeaderVisitor, NS_IHTTPHEADERVISITOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPHEADERVISITOR \
  [[nodiscard]] NS_IMETHOD VisitHeader(const nsACString& aHeader, const nsACString& aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPHEADERVISITOR \
  [[nodiscard]] nsresult VisitHeader(const nsACString& aHeader, const nsACString& aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPHEADERVISITOR(_to) \
  [[nodiscard]] NS_IMETHOD VisitHeader(const nsACString& aHeader, const nsACString& aValue) override { return _to VisitHeader(aHeader, aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPHEADERVISITOR(_to) \
  [[nodiscard]] NS_IMETHOD VisitHeader(const nsACString& aHeader, const nsACString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->VisitHeader(aHeader, aValue); } 


#endif /* __gen_nsIHttpHeaderVisitor_h__ */
