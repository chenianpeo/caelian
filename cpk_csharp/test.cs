using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

public class Person
{
    public string Name { get; set; } = string.Empty;
    public string Version { get; set; } = string.Empty;
    public string Url { get; set; } = string.Empty;
}

public class YamlReader
{
    public static List<Person> ReadPackages(string filePath)
    {
        if (string.IsNullOrWhiteSpace(filePath))
            throw new ArgumentException("filePath is null or empty", nameof(filePath));

        if (!File.Exists(filePath))
            throw new FileNotFoundException($"YAML file not found: {filePath}", filePath);

        var deserializer = new DeserializerBuilder()
            .WithNamingConvention(CamelCaseNamingConvention.Instance)
            .Build();

        var yamlContent = File.ReadAllText(filePath);

        try
        {
            var packages = new List<Person>();
            
            // Parse multiple YAML documents separated by ---
            using (var reader = new StringReader(yamlContent))
            {
                var yamlStream = new YamlDotNet.RepresentationModel.YamlStream();
                yamlStream.Load(reader);
                
                foreach (var document in yamlStream.Documents)
                {
                    if (document.RootNode is YamlDotNet.RepresentationModel.YamlMappingNode mapping)
                    {
                        var package = new Person();
                        
                        foreach (var entry in mapping.Children)
                        {
                            var key = ((YamlDotNet.RepresentationModel.YamlScalarNode)entry.Key).Value;
                            
                            if (entry.Value is YamlDotNet.RepresentationModel.YamlScalarNode scalar)
                            {
                                var value = scalar.Value;
                                switch (key?.ToLower())
                                {
                                    case "name":
                                        package.Name = value ?? string.Empty;
                                        break;
                                    case "version":
                                        package.Version = value ?? string.Empty;
                                        break;
                                    case "url":
                                        package.Url = value ?? string.Empty;
                                        break;
                                }
                            }
                        }
                        
                        if (!string.IsNullOrWhiteSpace(package.Name))
                        {
                            packages.Add(package);
                        }
                    }
                }
            }

            return packages;
        }
        catch (Exception ex)
        {
            throw new InvalidOperationException($"Failed to read or parse YAML file '{filePath}': {ex.Message}", ex);
        }
    }
}