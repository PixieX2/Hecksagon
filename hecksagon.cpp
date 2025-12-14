#include <bits/stdc++.h>
using namespace std;

int main(int argc, char** argv) {
    if (argc < 2) {
        cout << "Usage: " << argv[0] << " file.h2\n";
        return 1;
    }

    ifstream f(argv[1], ios::binary);
    if (!f) {
        cerr << "Cannot open file.\n";
        return 1;
    }

    string code((istreambuf_iterator<char>(f)), {});
    
    int cell = 0, nose = 0, shadow = 0, ghost = 0;
    bool debug = false;

    auto wrap = [](int x) {
        return (x % 256 + 256) % 256;
    };

    for (char cmd : code) {
        int dC = 0, dS = 0, dG = 0, flip = 0;

        switch (cmd) {
            case '!': dC=3;  dS=1;  break;
            case '@': dC=-7; flip=1; dS=2;  dG=1; break;
            case '#': dC=5;  dS=-1; dG=3; break;
            case '$': break;
            case '~': flip=1; break;
            case '%': dC=1;  dS=3;  dG=-2; break;
            case '^': dC=-3; dG=5; break;
            case '&': dC=2;  flip=1; dS=-2; break;
            case '*': dC=4;  dS=1;  dG=1; break;
            case '(': dC=-2; flip=1; dG=-1; break;
            case ')': dS=4; break;
            case '_': dG=1; break;
            case '+': dC=7;  dS=-3; dG=2; break;
            default: continue;
        }

        cell   = wrap(cell + dC);
        shadow = wrap(shadow + dS);
        ghost  = wrap(ghost + dG);
        if (flip) nose ^= 1;

        if (cmd == '$') {
            unsigned char out = (cell + shadow + ghost) & 0xFF;
            cout << out;
        }

        if (debug) {
            cout << "\n[DEBUG] cmd=" << cmd
                 << " cell=" << cell
                 << " shadow=" << shadow
                 << " ghost=" << ghost
                 << " nose=" << nose << "\n";
        }
    }

    for (int i = 0; i < ghost; i++) {
        cell   = wrap(cell + i*i + shadow - ghost);
        shadow = wrap(shadow + i*2 - nose);
        ghost  = wrap(ghost + i*3 + nose);
    }

    volatile int checksum = (cell + shadow + ghost + nose) & 0xFF;
    (void)checksum;

    return 0;
}
