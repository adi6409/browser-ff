/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINestedURI.idl
 */

#ifndef __gen_nsINestedURI_h__
#define __gen_nsINestedURI_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsINestedURI */
#define NS_INESTEDURI_IID_STR "6de2c874-796c-46bf-b57f-0d7bd7d6cab0"

#define NS_INESTEDURI_IID \
  {0x6de2c874, 0x796c, 0x46bf, \
    { 0xb5, 0x7f, 0x0d, 0x7b, 0xd7, 0xd6, 0xca, 0xb0 }}

class NS_NO_VTABLE nsINestedURI : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INESTEDURI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINestedURI;

  /* readonly attribute nsIURI innerURI; */
  NS_IMETHOD GetInnerURI(nsIURI **aInnerURI) = 0;

  /* readonly attribute nsIURI innermostURI; */
  NS_IMETHOD GetInnermostURI(nsIURI **aInnermostURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINestedURI, NS_INESTEDURI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINESTEDURI \
  NS_IMETHOD GetInnerURI(nsIURI **aInnerURI) override; \
  NS_IMETHOD GetInnermostURI(nsIURI **aInnermostURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINESTEDURI \
  nsresult GetInnerURI(nsIURI **aInnerURI); \
  nsresult GetInnermostURI(nsIURI **aInnermostURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINESTEDURI(_to) \
  NS_IMETHOD GetInnerURI(nsIURI **aInnerURI) override { return _to GetInnerURI(aInnerURI); } \
  NS_IMETHOD GetInnermostURI(nsIURI **aInnermostURI) override { return _to GetInnermostURI(aInnermostURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINESTEDURI(_to) \
  NS_IMETHOD GetInnerURI(nsIURI **aInnerURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInnerURI(aInnerURI); } \
  NS_IMETHOD GetInnermostURI(nsIURI **aInnermostURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInnermostURI(aInnermostURI); } 


/* starting interface:    nsINestedURIMutator */
#define NS_INESTEDURIMUTATOR_IID_STR "ca3d6c03-4eee-4271-a97a-d16c0a0b2c5c"

#define NS_INESTEDURIMUTATOR_IID \
  {0xca3d6c03, 0x4eee, 0x4271, \
    { 0xa9, 0x7a, 0xd1, 0x6c, 0x0a, 0x0b, 0x2c, 0x5c }}

class NS_NO_VTABLE nsINestedURIMutator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INESTEDURIMUTATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINestedURIMutator;

  /* [must_use,noscript] void init (in nsIURI innerURI); */
  [[nodiscard]] NS_IMETHOD Init(nsIURI *innerURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINestedURIMutator, NS_INESTEDURIMUTATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINESTEDURIMUTATOR \
  [[nodiscard]] NS_IMETHOD Init(nsIURI *innerURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINESTEDURIMUTATOR \
  [[nodiscard]] nsresult Init(nsIURI *innerURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINESTEDURIMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD Init(nsIURI *innerURI) override { return _to Init(innerURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINESTEDURIMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD Init(nsIURI *innerURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(innerURI); } 


/* starting interface:    nsINestedAboutURIMutator */
#define NS_INESTEDABOUTURIMUTATOR_IID_STR "c6357a3b-c2bb-4b4b-9278-513377398a38"

#define NS_INESTEDABOUTURIMUTATOR_IID \
  {0xc6357a3b, 0xc2bb, 0x4b4b, \
    { 0x92, 0x78, 0x51, 0x33, 0x77, 0x39, 0x8a, 0x38 }}

class NS_NO_VTABLE nsINestedAboutURIMutator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INESTEDABOUTURIMUTATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINestedAboutURIMutator;

  /* [must_use,noscript] void initWithBase (in nsIURI innerURI, in nsIURI baseURI); */
  [[nodiscard]] NS_IMETHOD InitWithBase(nsIURI *innerURI, nsIURI *baseURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINestedAboutURIMutator, NS_INESTEDABOUTURIMUTATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINESTEDABOUTURIMUTATOR \
  [[nodiscard]] NS_IMETHOD InitWithBase(nsIURI *innerURI, nsIURI *baseURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINESTEDABOUTURIMUTATOR \
  [[nodiscard]] nsresult InitWithBase(nsIURI *innerURI, nsIURI *baseURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINESTEDABOUTURIMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD InitWithBase(nsIURI *innerURI, nsIURI *baseURI) override { return _to InitWithBase(innerURI, baseURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINESTEDABOUTURIMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD InitWithBase(nsIURI *innerURI, nsIURI *baseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitWithBase(innerURI, baseURI); } 


/* starting interface:    nsIJSURIMutator */
#define NS_IJSURIMUTATOR_IID_STR "3bd44535-08ea-478f-99b9-85fa1084e820"

#define NS_IJSURIMUTATOR_IID \
  {0x3bd44535, 0x08ea, 0x478f, \
    { 0x99, 0xb9, 0x85, 0xfa, 0x10, 0x84, 0xe8, 0x20 }}

class NS_NO_VTABLE nsIJSURIMutator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IJSURIMUTATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIJSURIMutator;

  /* [must_use,noscript] void setBase (in nsIURI aBaseURI); */
  [[nodiscard]] NS_IMETHOD SetBase(nsIURI *aBaseURI) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIJSURIMutator, NS_IJSURIMUTATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIJSURIMUTATOR \
  [[nodiscard]] NS_IMETHOD SetBase(nsIURI *aBaseURI) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIJSURIMUTATOR \
  [[nodiscard]] nsresult SetBase(nsIURI *aBaseURI); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIJSURIMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD SetBase(nsIURI *aBaseURI) override { return _to SetBase(aBaseURI); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIJSURIMUTATOR(_to) \
  [[nodiscard]] NS_IMETHOD SetBase(nsIURI *aBaseURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBase(aBaseURI); } 


#endif /* __gen_nsINestedURI_h__ */
