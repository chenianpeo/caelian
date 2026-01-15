using System;
using System.Runtime.InteropServices;

class Program
{
    [DllImport(@"d:\Documents\Code\Projects\caelian\cpk_rust\target\debug\cpk_rust.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void ffi_hello
        ([MarshalAs(UnmanagedType.LPStr)] string name);

    static void Main()
    {
        Console.WriteLine("Calling Rust function from C#:");
        ffi_hello("lsr");
        Console.WriteLine("\nPress any key to exit...");
        Console.ReadKey();
    }
}