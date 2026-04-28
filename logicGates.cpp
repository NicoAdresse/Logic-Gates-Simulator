#include <iostream>
// 0 = False
// 1 = True

bool AND(bool a, bool b) { return a && b; }
bool OR(bool a, bool b) { return a || b; }
bool NOT(bool a) { return !a; }
bool XOR(bool a, bool b) { return (a ^ b); }
bool NAND(bool a, bool b) { return !(a && b); }
bool NOR(bool a, bool b) { return !(a || b); }
bool XNOR(bool a, bool b) { return (a == b); }

bool GetBoolInput(const std::string& prompt) {
    int value;
    while (true) {
        std::cout << prompt;
        std::cin >> value;

        if (std::cin && (value == 0 || value == 1)) {
            return value == 1;
        }
        
        std::cin.clear(); std::cin.ignore(10000, '\n');
        std::cout << "Invalid input. Enter 0 or 1.\n";
    }
}

// And Gate
void RunANDGate() {
    bool a = GetBoolInput("Input 1 (0/1): ");
    bool b = GetBoolInput("Input 2 (0/1): ");
    std::cout << "AND result: " << AND(a, b) << "\n";
}

// Or Gate
void RunORGate() {
    bool a = GetBoolInput("Input 1 (0/1): ");
    bool b = GetBoolInput("Input 2 (0/1): ");
    std::cout << "OR result: " << OR(a, b) << "\n";
}

// Not Gate
void RunNOTGate() {
    bool a = GetBoolInput("Input !ONLY! 1 (0/1): ");
    std::cout << "NOT result: " << NOT(a) << "\n";
}

// Xor Gate
void RunXORGate() {
    bool a = GetBoolInput("Input 1 (0/1): ");
    bool b = GetBoolInput("Input 2 (0/1): ");
    std::cout << "XOR result: " << XOR(a, b) << "\n";
}

// Nand Gate
void RunNANDGate() {
    bool a = GetBoolInput("Input 1 (0/1): ");
    bool b = GetBoolInput("Input 2 (0/1): ");
    std::cout << "NAND result: " << NAND(a, b) << "\n";
}

// Nor Gate
void RunNORGate() {
    bool a = GetBoolInput("Input 1 (0/1): ");
    bool b = GetBoolInput("Input 2 (0/1): ");
    std::cout << "NOR result: " << NOR(a, b) << "\n";
}

// Xnor Gate
void RunXNORGate() {
    bool a = GetBoolInput("Input 1 (0/1): ");
    bool b = GetBoolInput("Input 2 (0/1): ");
    std::cout << "XNOR result: " << XNOR(a, b) << "\n";
}

int main() {
    bool IsRunning = true;
    while (IsRunning) {
        std::cout << "1. AND\n2. OR\n3. NOT\n4. XOR" <<
        "\n5. NAND\n6. NOR\n7. XNOR\n8. Exit\n";

        int choice;
        std::cin >> choice;

        switch (choice) {
            case 1: RunANDGate(); break;
            case 2: RunORGate(); break;
            case 3: RunNOTGate(); break;
            case 4: RunXORGate(); break;
            case 5: RunNANDGate(); break;
            case 6: RunNORGate(); break;
            case 7: RunXNORGate(); break;
            case 8: IsRunning = false; break;
            default: continue;

        }
    }

    std::cin.ignore();
    std::cin.get();
    return 0;
}
