using System.IO;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

using YamlDotNet.Serialization;
using YamlDotNet.Serialization.NamingConventions;

namespace cael_csharp
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
        }

        private void Button_Click(object sender, RoutedEventArgs e)
        {
            Applications content = Functions.ReadYaml();
            boxPrint.Items.Add($"Name: {content.Name}");
            boxPrint.Items.Add($"Version: {content.Version}");
            boxPrint.Items.Add($"Size: {content.Size}");
            boxPrint.Items.Add($"Bytes: {content.Bytes}");
            boxPrint.Items.Add($"Descriptions: {content.Description}");
            boxPrint.Items.Add($"Purpose: {content.Purpose}");
        }

        private void Button_Click_1(object sender, RoutedEventArgs e)
        {
            boxPrint.Items.Clear();
        }
    }

    public class Applications
    {
        public string Name { get; set; }
        public string Version { get; set; }
        public string Size { get; set; }
        public string Bytes { get; set; }
        public string Description { get; set; }
        public string Purpose { get; set; }
    }
    public class Functions
    {
        public static Applications ReadYaml()
        {
            //var app = new Applications
            //{
            //    Name = "Abe",
            //};
            //var serializer = new SerializerBuilder().WithNamingConvention(CamelCaseNamingConvention.Instance).Build();
            //var yaml = serializer.Serialize(app);
            //return yaml;

            var app = File.ReadAllText("packages.yaml");
            var deserializer = new DeserializerBuilder().WithNamingConvention(CamelCaseNamingConvention.Instance).Build();
            var test = deserializer.Deserialize<Applications>(app);
            return test;
        }
    }
}