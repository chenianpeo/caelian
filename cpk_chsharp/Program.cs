using System;
using System.Runtime.InteropServices;
using System.IO;

class Program
{
    private const string DllName = "cpk_rust.dll";
    
    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void ffi_hello
        ([MarshalAs(UnmanagedType.LPStr)] string name);
    
    static Program()
    {
        string dllPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, DllName);
        if (!File.Exists(dllPath))
        {
            Console.WriteLine($"Warning: DLL not found at {dllPath}");
        }
    }

    static void Main()
    {
        Console.WriteLine("Calling rust function from C#:");
        ffi_hello("lsr");
        Console.WriteLine("\nPress any key to exit...");
        Console.ReadKey();
    }
}