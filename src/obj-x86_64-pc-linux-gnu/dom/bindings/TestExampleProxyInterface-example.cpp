/* -*- Mode: C++; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* vim:set ts=2 sw=2 sts=2 et cindent: */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#include "mozilla/dom/TestExampleGenBinding.h"
#include "mozilla/dom/TestExampleProxyInterface.h"

namespace mozilla {
namespace dom {


// Only needed for refcounted objects.
NS_IMPL_CYCLE_COLLECTION_WRAPPERCACHE_0(TestExampleProxyInterface)
NS_IMPL_CYCLE_COLLECTING_ADDREF(TestExampleProxyInterface)
NS_IMPL_CYCLE_COLLECTING_RELEASE(TestExampleProxyInterface)
NS_INTERFACE_MAP_BEGIN_CYCLE_COLLECTION(TestExampleProxyInterface)
  NS_WRAPPERCACHE_INTERFACE_MAP_ENTRY
  NS_INTERFACE_MAP_ENTRY(nsISupports)
NS_INTERFACE_MAP_END

TestExampleProxyInterface::TestExampleProxyInterface()
{
    // Add |MOZ_COUNT_CTOR(TestExampleProxyInterface);| for a non-refcounted object.
}

TestExampleProxyInterface::~TestExampleProxyInterface()
{
    // Add |MOZ_COUNT_DTOR(TestExampleProxyInterface);| for a non-refcounted object.
}

JSObject*
TestExampleProxyInterface::WrapObject(JSContext* aCx, JS::Handle<JSObject*> aGivenProto)
{
  return TestExampleProxyInterface_Binding::Wrap(aCx, this, aGivenProto);
}


} // namespace dom
} // namespace mozilla
