#include <windows.h>

// Dimensions de la fenêtre
#define WIDTH 700
#define HEIGHT 400

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    switch (uMsg) {
        case WM_DESTROY:
            PostQuitMessage(0);
            return 0;

        case WM_COMMAND:
            if (LOWORD(wParam) == 1) {
                MessageBox(hwnd, L"Tu as cliqué sur le bouton !", L"Info", MB_OK | MB_ICONINFORMATION);
            }
            break;
    }
    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE, LPSTR, int nCmdShow) {
    const wchar_t CLASS_NAME[] = L"MyWindowClass";

    WNDCLASS wc = {};
    wc.lpfnWndProc   = WindowProc;
    wc.hInstance     = hInstance;
    wc.lpszClassName = CLASS_NAME;
    wc.hbrBackground = (HBRUSH)(COLOR_WINDOW+1);

    RegisterClass(&wc);

    // Centrage
    int screenX = (GetSystemMetrics(SM_CXSCREEN) - WIDTH) / 2;
    int screenY = (GetSystemMetrics(SM_CYSCREEN) - HEIGHT) / 2;

    HWND hwnd = CreateWindowEx(
        0,
        CLASS_NAME,
        L"NM Launcher",
        WS_OVERLAPPEDWINDOW & ~WS_MAXIMIZEBOX & ~WS_SIZEBOX,
        screenX, screenY, WIDTH, HEIGHT,
        nullptr, nullptr, hInstance, nullptr
    );

    if (hwnd == nullptr) return 0;

    // Bouton
    CreateWindow(
        L"BUTTON",
        L"Lancer",
        WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON,
        290, 170, 120, 40,
        hwnd, (HMENU)1,
        hInstance, nullptr
    );

    ShowWindow(hwnd, nCmdShow);

    MSG msg = {};
    while (GetMessage(&msg, nullptr, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return 0;
}
