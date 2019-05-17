using System;

namespace DotnetFrameworkChecker
{
    class Program
    {
        public static void Main()
        {
            Console.WriteLine("- check 4.5 later version.");
            GetDotNetVersion.Get45PlusFromRegistry();
            Console.WriteLine();

            Console.WriteLine("- check 1-4 version.");
            GetDotNetVersion.GetVersionFromRegistry();

            Console.ReadLine();
        }
    }
}
