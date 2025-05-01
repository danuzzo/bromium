//******************************************************************************
//
// Copyright (c) 2018 Microsoft Corporation. All rights reserved.
//
// This code is licensed under the MIT License (MIT).
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//
//******************************************************************************

#include "stdafx.h"
#include "UiTreeWalk.h"

// Export functions for static linking - these will be called from Rust
extern "C" {
    // Export functions with C linkage to avoid name mangling
    extern "C" __declspec(dllexport) void InitUiTreeWalk() {
        UiTreeWalk::Init();
    }

    extern "C" __declspec(dllexport) void UnInitUiTreeWalk() {
        UiTreeWalk::UnInit();
    }

    extern "C" __declspec(dllexport) long GetUiXPath(long left, long top, LPWSTR lpUiPath, long nMaxCount) {
        return UiTreeWalk::GetUiXPath(left, top, lpUiPath, nMaxCount);
    }

    extern "C" __declspec(dllexport) long HighlightCachedUI(LPWSTR lpRumtimeId, RECT* pRect) {
        return UiTreeWalk::HighlightCachedUI(lpRumtimeId, pRect);
    }
}