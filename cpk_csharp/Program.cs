using System;
using System.IO;
using System.Runtime.InteropServices;

class Program
{
    [DllImport("cpk_rust.dll", CallingConvention = CallingConvention.Cdecl)]
    private static extern void ffi_packages_list_obtain();

    static void Main()
    {
        // Read YAML file
        string yamlPath = @"D:\Documents\Code\Projects\caelian\cpk_rust\src\packages_data\packages.yaml";

        try
        {
            var packages = YamlReader.ReadPackages(yamlPath) ?? new System.Collections.Generic.List<Person>();

            // Display the packages
            foreach (var package in packages)
            {
                Console.WriteLine($"Name: {package.Name}");
                Console.WriteLine($"Version: {package.Version}");
                Console.WriteLine($"Url: {package.Url}");
                Console.WriteLine("---");
            }
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine($"Error reading packages YAML: {ex.Message}");
            Environment.ExitCode = 1;
            return;
        }

        //ffi_packages_list_obtain();
    }
}