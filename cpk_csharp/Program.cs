using System;
using System.IO;
using System.Runtime.InteropServices;

class Program
{
    [DllImport("cpk_rust.dll", CallingConvention = CallingConvention.Cdecl)]
    private static extern void ffi_packages_list_obtain();

    static void Main()
    {
        ffi_packages_list_obtain();
    }
}