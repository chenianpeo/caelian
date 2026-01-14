using System;
using System.Runtime.InteropServices;
using System.Security;

namespace TestDllCSharp
{
    /// rust dll function import class
    public static class TestDll
    {
        private const string DllName = "dll_test";

        /// <summary>
        /// int add
        /// </summary>
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern int add(int a, int b);

        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        public static extern double add_float(double a, double b);
    }

    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("==== test rust dll example ====\n");

            try
            {
                int result1 = TestDll.add(10, 20);
                Console.WriteLine($"10 + 20 = {result1}");
                int result2 = TestDll.add(-5, 15);
                Console.WriteLine($"-5 + 15 = {result2}");

                double result3 = TestDll.add_float(3.14, 2.86);
                Console.WriteLine($"3.14 + 2.86 = {result3}");
                double result4 = TestDll.add_float(1.5, 2.5);
                Console.WriteLine($"1.5 + 2.5 = {result4}");
            }
            catch (DllNotFoundException ex)
            {
                Console.WriteLine($"error: not found dll file");
                Console.WriteLine($"ensure test_dll.dll under app folder");
                Console.WriteLine($"details info: {ex.Message}");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"error: {ex.Message}");
            }

            Console.WriteLine("\n press any key to exit");
            Console.ReadKey();
        }
    }
}