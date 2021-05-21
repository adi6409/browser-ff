/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICertTree.idl
 */

#ifndef __gen_nsICertTree_h__
#define __gen_nsICertTree_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsITreeView_h__
#include "nsITreeView.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIX509Cert; /* forward declaration */


/* starting interface:    nsICertTreeItem */
#define NS_ICERTTREEITEM_IID_STR "d0180863-606e-49e6-8324-cf45ed4dd891"

#define NS_ICERTTREEITEM_IID \
  {0xd0180863, 0x606e, 0x49e6, \
    { 0x83, 0x24, 0xcf, 0x45, 0xed, 0x4d, 0xd8, 0x91 }}

class NS_NO_VTABLE nsICertTreeItem : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTTREEITEM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertTreeItem;

  /* [must_use] readonly attribute nsIX509Cert cert; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetCert(nsIX509Cert **aCert) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertTreeItem, NS_ICERTTREEITEM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTTREEITEM \
  [[nodiscard]] NS_IMETHOD GetCert(nsIX509Cert **aCert) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTTREEITEM \
  [[nodiscard]] nsresult GetCert(nsIX509Cert **aCert); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTTREEITEM(_to) \
  [[nodiscard]] NS_IMETHOD GetCert(nsIX509Cert **aCert) override { return _to GetCert(aCert); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTTREEITEM(_to) \
  [[nodiscard]] NS_IMETHOD GetCert(nsIX509Cert **aCert) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCert(aCert); } 


/* starting interface:    nsICertTree */
#define NS_ICERTTREE_IID_STR "55d5ad6b-5572-47fe-941c-f01fe723659e"

#define NS_ICERTTREE_IID \
  {0x55d5ad6b, 0x5572, 0x47fe, \
    { 0x94, 0x1c, 0xf0, 0x1f, 0xe7, 0x23, 0x65, 0x9e }}

class NS_NO_VTABLE nsICertTree : public nsITreeView {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICERTTREE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICertTree;

  /* [must_use] void loadCertsFromCache (in Array<nsIX509Cert> cache, in unsigned long type); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD LoadCertsFromCache(const nsTArray<RefPtr<nsIX509Cert>>& cache, uint32_t type) = 0;

  /* [must_use] nsIX509Cert getCert (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetCert(uint32_t index, nsIX509Cert **_retval) = 0;

  /* [must_use] nsICertTreeItem getTreeItem (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetTreeItem(uint32_t index, nsICertTreeItem **_retval) = 0;

  /* [must_use] void deleteEntryObject (in unsigned long index); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD DeleteEntryObject(uint32_t index) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICertTree, NS_ICERTTREE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICERTTREE \
  [[nodiscard]] NS_IMETHOD LoadCertsFromCache(const nsTArray<RefPtr<nsIX509Cert>>& cache, uint32_t type) override; \
  [[nodiscard]] NS_IMETHOD GetCert(uint32_t index, nsIX509Cert **_retval) override; \
  [[nodiscard]] NS_IMETHOD GetTreeItem(uint32_t index, nsICertTreeItem **_retval) override; \
  [[nodiscard]] NS_IMETHOD DeleteEntryObject(uint32_t index) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICERTTREE \
  [[nodiscard]] nsresult LoadCertsFromCache(const nsTArray<RefPtr<nsIX509Cert>>& cache, uint32_t type); \
  [[nodiscard]] nsresult GetCert(uint32_t index, nsIX509Cert **_retval); \
  [[nodiscard]] nsresult GetTreeItem(uint32_t index, nsICertTreeItem **_retval); \
  [[nodiscard]] nsresult DeleteEntryObject(uint32_t index); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICERTTREE(_to) \
  [[nodiscard]] NS_IMETHOD LoadCertsFromCache(const nsTArray<RefPtr<nsIX509Cert>>& cache, uint32_t type) override { return _to LoadCertsFromCache(cache, type); } \
  [[nodiscard]] NS_IMETHOD GetCert(uint32_t index, nsIX509Cert **_retval) override { return _to GetCert(index, _retval); } \
  [[nodiscard]] NS_IMETHOD GetTreeItem(uint32_t index, nsICertTreeItem **_retval) override { return _to GetTreeItem(index, _retval); } \
  [[nodiscard]] NS_IMETHOD DeleteEntryObject(uint32_t index) override { return _to DeleteEntryObject(index); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICERTTREE(_to) \
  [[nodiscard]] NS_IMETHOD LoadCertsFromCache(const nsTArray<RefPtr<nsIX509Cert>>& cache, uint32_t type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->LoadCertsFromCache(cache, type); } \
  [[nodiscard]] NS_IMETHOD GetCert(uint32_t index, nsIX509Cert **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCert(index, _retval); } \
  [[nodiscard]] NS_IMETHOD GetTreeItem(uint32_t index, nsICertTreeItem **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTreeItem(index, _retval); } \
  [[nodiscard]] NS_IMETHOD DeleteEntryObject(uint32_t index) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteEntryObject(index); } 


#define NS_CERTTREE_CID { 0x4ea60761, 0x31d6, 0x491d, \
                         { 0x9e, 0x34, 0x4b, 0x53, 0xa2, 0x6c, 0x41, 0x6c } }
#define NS_CERTTREE_CONTRACTID "@mozilla.org/security/nsCertTree;1"

#endif /* __gen_nsICertTree_h__ */
