/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocShellTreeItem.idl
 */

#ifndef __gen_nsIDocShellTreeItem_h__
#define __gen_nsIDocShellTreeItem_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */

class nsIDocShellTreeOwner; /* forward declaration */

class nsPIDOMWindowOuter; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDocShellTreeItem */
#define NS_IDOCSHELLTREEITEM_IID_STR "9b7c586f-9214-480c-a2c4-49b526fff1a6"

#define NS_IDOCSHELLTREEITEM_IID \
  {0x9b7c586f, 0x9214, 0x480c, \
    { 0xa2, 0xc4, 0x49, 0xb5, 0x26, 0xff, 0xf1, 0xa6 }}

class NS_NO_VTABLE nsIDocShellTreeItem : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOCSHELLTREEITEM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDocShellTreeItem;

  /* attribute AString name; */
  NS_IMETHOD GetName(nsAString& aName) = 0;
  NS_IMETHOD SetName(const nsAString& aName) = 0;

  /* boolean nameEquals (in AString name); */
  NS_IMETHOD NameEquals(const nsAString& name, bool *_retval) = 0;

  enum {
    typeChrome = 0,
    typeContent = 1,
    typeContentWrapper = 2,
    typeChromeWrapper = 3,
    typeAll = 2147483647
  };

  /* readonly attribute long itemType; */
  NS_IMETHOD GetItemType(int32_t *aItemType) = 0;

  /* [noscript,nostdcall,notxpcom] long ItemType (); */
  virtual int32_t ItemType(void) = 0;

  /* [binaryname(InProcessParent)] readonly attribute nsIDocShellTreeItem parent; */
  NS_IMETHOD GetInProcessParent(nsIDocShellTreeItem **aParent) = 0;

  /* [binaryname(InProcessSameTypeParent)] readonly attribute nsIDocShellTreeItem sameTypeParent; */
  NS_IMETHOD GetInProcessSameTypeParent(nsIDocShellTreeItem **aSameTypeParent) = 0;

  /* [binaryname(InProcessRootTreeItem)] readonly attribute nsIDocShellTreeItem rootTreeItem; */
  NS_IMETHOD GetInProcessRootTreeItem(nsIDocShellTreeItem **aRootTreeItem) = 0;

  /* [binaryname(InProcessSameTypeRootTreeItem)] readonly attribute nsIDocShellTreeItem sameTypeRootTreeItem; */
  NS_IMETHOD GetInProcessSameTypeRootTreeItem(nsIDocShellTreeItem **aSameTypeRootTreeItem) = 0;

  /* readonly attribute nsIDocShellTreeOwner treeOwner; */
  NS_IMETHOD GetTreeOwner(nsIDocShellTreeOwner **aTreeOwner) = 0;

  /* [noscript] void setTreeOwner (in nsIDocShellTreeOwner treeOwner); */
  NS_IMETHOD SetTreeOwner(nsIDocShellTreeOwner *treeOwner) = 0;

  /* [binaryname(InProcessChildCount),infallible] readonly attribute long childCount; */
  NS_IMETHOD GetInProcessChildCount(int32_t *aChildCount) = 0;
  inline int32_t  GetInProcessChildCount()
  {
    int32_t result;
    mozilla::DebugOnly<nsresult> rv = GetInProcessChildCount(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [noscript] void addChild (in nsIDocShellTreeItem child); */
  NS_IMETHOD AddChild(nsIDocShellTreeItem *child) = 0;

  /* [noscript] void removeChild (in nsIDocShellTreeItem child); */
  NS_IMETHOD RemoveChild(nsIDocShellTreeItem *child) = 0;

  /* [binaryname(GetInProcessChildAt)] nsIDocShellTreeItem getChildAt (in long index); */
  NS_IMETHOD GetInProcessChildAt(int32_t index, nsIDocShellTreeItem **_retval) = 0;

  /* [binaryname(BrowsingContextXPCOM)] readonly attribute BrowsingContext browsingContext; */
  NS_IMETHOD GetBrowsingContextXPCOM(mozilla::dom::BrowsingContext **aBrowsingContext) = 0;

  /* [noscript,nostdcall,notxpcom] BrowsingContext getBrowsingContext (); */
  virtual mozilla::dom::BrowsingContext * GetBrowsingContext(void) = 0;

  /* readonly attribute mozIDOMWindowProxy domWindow; */
  NS_IMETHOD GetDomWindow(mozIDOMWindowProxy **aDomWindow) = 0;

  /* [noscript,nostdcall,notxpcom] Document getDocument (); */
  virtual mozilla::dom::Document * GetDocument(void) = 0;

  /* [noscript,nostdcall,notxpcom] nsPIDOMWindowOuter getWindow (); */
  virtual nsPIDOMWindowOuter * GetWindow(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDocShellTreeItem, NS_IDOCSHELLTREEITEM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOCSHELLTREEITEM \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD SetName(const nsAString& aName) override; \
  NS_IMETHOD NameEquals(const nsAString& name, bool *_retval) override; \
  NS_IMETHOD GetItemType(int32_t *aItemType) override; \
  virtual int32_t ItemType(void) override; \
  NS_IMETHOD GetInProcessParent(nsIDocShellTreeItem **aParent) override; \
  NS_IMETHOD GetInProcessSameTypeParent(nsIDocShellTreeItem **aSameTypeParent) override; \
  NS_IMETHOD GetInProcessRootTreeItem(nsIDocShellTreeItem **aRootTreeItem) override; \
  NS_IMETHOD GetInProcessSameTypeRootTreeItem(nsIDocShellTreeItem **aSameTypeRootTreeItem) override; \
  NS_IMETHOD GetTreeOwner(nsIDocShellTreeOwner **aTreeOwner) override; \
  NS_IMETHOD SetTreeOwner(nsIDocShellTreeOwner *treeOwner) override; \
  using nsIDocShellTreeItem::GetInProcessChildCount; \
  NS_IMETHOD GetInProcessChildCount(int32_t *aChildCount) override; \
  NS_IMETHOD AddChild(nsIDocShellTreeItem *child) override; \
  NS_IMETHOD RemoveChild(nsIDocShellTreeItem *child) override; \
  NS_IMETHOD GetInProcessChildAt(int32_t index, nsIDocShellTreeItem **_retval) override; \
  NS_IMETHOD GetBrowsingContextXPCOM(mozilla::dom::BrowsingContext **aBrowsingContext) override; \
  virtual mozilla::dom::BrowsingContext * GetBrowsingContext(void) override; \
  NS_IMETHOD GetDomWindow(mozIDOMWindowProxy **aDomWindow) override; \
  virtual mozilla::dom::Document * GetDocument(void) override; \
  virtual nsPIDOMWindowOuter * GetWindow(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOCSHELLTREEITEM \
  nsresult GetName(nsAString& aName); \
  nsresult SetName(const nsAString& aName); \
  nsresult NameEquals(const nsAString& name, bool *_retval); \
  nsresult GetItemType(int32_t *aItemType); \
  int32_t ItemType(void); \
  nsresult GetInProcessParent(nsIDocShellTreeItem **aParent); \
  nsresult GetInProcessSameTypeParent(nsIDocShellTreeItem **aSameTypeParent); \
  nsresult GetInProcessRootTreeItem(nsIDocShellTreeItem **aRootTreeItem); \
  nsresult GetInProcessSameTypeRootTreeItem(nsIDocShellTreeItem **aSameTypeRootTreeItem); \
  nsresult GetTreeOwner(nsIDocShellTreeOwner **aTreeOwner); \
  nsresult SetTreeOwner(nsIDocShellTreeOwner *treeOwner); \
  using nsIDocShellTreeItem::GetInProcessChildCount; \
  nsresult GetInProcessChildCount(int32_t *aChildCount); \
  nsresult AddChild(nsIDocShellTreeItem *child); \
  nsresult RemoveChild(nsIDocShellTreeItem *child); \
  nsresult GetInProcessChildAt(int32_t index, nsIDocShellTreeItem **_retval); \
  nsresult GetBrowsingContextXPCOM(mozilla::dom::BrowsingContext **aBrowsingContext); \
  mozilla::dom::BrowsingContext * GetBrowsingContext(void); \
  nsresult GetDomWindow(mozIDOMWindowProxy **aDomWindow); \
  mozilla::dom::Document * GetDocument(void); \
  nsPIDOMWindowOuter * GetWindow(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOCSHELLTREEITEM(_to) \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD SetName(const nsAString& aName) override { return _to SetName(aName); } \
  NS_IMETHOD NameEquals(const nsAString& name, bool *_retval) override { return _to NameEquals(name, _retval); } \
  NS_IMETHOD GetItemType(int32_t *aItemType) override { return _to GetItemType(aItemType); } \
  virtual int32_t ItemType(void) override { return _to ItemType(); } \
  NS_IMETHOD GetInProcessParent(nsIDocShellTreeItem **aParent) override { return _to GetInProcessParent(aParent); } \
  NS_IMETHOD GetInProcessSameTypeParent(nsIDocShellTreeItem **aSameTypeParent) override { return _to GetInProcessSameTypeParent(aSameTypeParent); } \
  NS_IMETHOD GetInProcessRootTreeItem(nsIDocShellTreeItem **aRootTreeItem) override { return _to GetInProcessRootTreeItem(aRootTreeItem); } \
  NS_IMETHOD GetInProcessSameTypeRootTreeItem(nsIDocShellTreeItem **aSameTypeRootTreeItem) override { return _to GetInProcessSameTypeRootTreeItem(aSameTypeRootTreeItem); } \
  NS_IMETHOD GetTreeOwner(nsIDocShellTreeOwner **aTreeOwner) override { return _to GetTreeOwner(aTreeOwner); } \
  NS_IMETHOD SetTreeOwner(nsIDocShellTreeOwner *treeOwner) override { return _to SetTreeOwner(treeOwner); } \
  using nsIDocShellTreeItem::GetInProcessChildCount; \
  NS_IMETHOD GetInProcessChildCount(int32_t *aChildCount) override { return _to GetInProcessChildCount(aChildCount); } \
  NS_IMETHOD AddChild(nsIDocShellTreeItem *child) override { return _to AddChild(child); } \
  NS_IMETHOD RemoveChild(nsIDocShellTreeItem *child) override { return _to RemoveChild(child); } \
  NS_IMETHOD GetInProcessChildAt(int32_t index, nsIDocShellTreeItem **_retval) override { return _to GetInProcessChildAt(index, _retval); } \
  NS_IMETHOD GetBrowsingContextXPCOM(mozilla::dom::BrowsingContext **aBrowsingContext) override { return _to GetBrowsingContextXPCOM(aBrowsingContext); } \
  virtual mozilla::dom::BrowsingContext * GetBrowsingContext(void) override { return _to GetBrowsingContext(); } \
  NS_IMETHOD GetDomWindow(mozIDOMWindowProxy **aDomWindow) override { return _to GetDomWindow(aDomWindow); } \
  virtual mozilla::dom::Document * GetDocument(void) override { return _to GetDocument(); } \
  virtual nsPIDOMWindowOuter * GetWindow(void) override { return _to GetWindow(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOCSHELLTREEITEM(_to) \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD SetName(const nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetName(aName); } \
  NS_IMETHOD NameEquals(const nsAString& name, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NameEquals(name, _retval); } \
  NS_IMETHOD GetItemType(int32_t *aItemType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetItemType(aItemType); } \
  virtual int32_t ItemType(void) override; \
  NS_IMETHOD GetInProcessParent(nsIDocShellTreeItem **aParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInProcessParent(aParent); } \
  NS_IMETHOD GetInProcessSameTypeParent(nsIDocShellTreeItem **aSameTypeParent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInProcessSameTypeParent(aSameTypeParent); } \
  NS_IMETHOD GetInProcessRootTreeItem(nsIDocShellTreeItem **aRootTreeItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInProcessRootTreeItem(aRootTreeItem); } \
  NS_IMETHOD GetInProcessSameTypeRootTreeItem(nsIDocShellTreeItem **aSameTypeRootTreeItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInProcessSameTypeRootTreeItem(aSameTypeRootTreeItem); } \
  NS_IMETHOD GetTreeOwner(nsIDocShellTreeOwner **aTreeOwner) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTreeOwner(aTreeOwner); } \
  NS_IMETHOD SetTreeOwner(nsIDocShellTreeOwner *treeOwner) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTreeOwner(treeOwner); } \
  NS_IMETHOD GetInProcessChildCount(int32_t *aChildCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInProcessChildCount(aChildCount); } \
  NS_IMETHOD AddChild(nsIDocShellTreeItem *child) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddChild(child); } \
  NS_IMETHOD RemoveChild(nsIDocShellTreeItem *child) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveChild(child); } \
  NS_IMETHOD GetInProcessChildAt(int32_t index, nsIDocShellTreeItem **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInProcessChildAt(index, _retval); } \
  NS_IMETHOD GetBrowsingContextXPCOM(mozilla::dom::BrowsingContext **aBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowsingContextXPCOM(aBrowsingContext); } \
  virtual mozilla::dom::BrowsingContext * GetBrowsingContext(void) override; \
  NS_IMETHOD GetDomWindow(mozIDOMWindowProxy **aDomWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomWindow(aDomWindow); } \
  virtual mozilla::dom::Document * GetDocument(void) override; \
  virtual nsPIDOMWindowOuter * GetWindow(void) override; 


#endif /* __gen_nsIDocShellTreeItem_h__ */
