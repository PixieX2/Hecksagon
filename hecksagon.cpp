#include <bits/stdc++.h>
using namespace std;

// Hecksagon 2.0 Interpreter
// Yes i skipped the 1.0 version... kinda 
// The reason it's called "Hecksagon 2.0" is because it's a bit more complex then the original
// 1.0 code was destroyed by the apocalyptic event

// Impossible for humans to write by hand
// Still fully generatable
int main(int argc, char** argv) {
    if(argc < 2){
        cout << "Usage: " << argv[0] << " file.h2\n";
        return 1;
    }

    ifstream f(argv[1]);
    if(!f.is_open()){
        cerr << "Cannot open file.\n";
        return 1;
    }

    string code((istreambuf_iterator<char>(f)), istreambuf_iterator<char>());
    
    int cell = 0;
    int nose = 0;
    int shadow = 0;
    int ghost = 0; // extra hidden register

    bool debug = false; // toggle debug mode
    
    // Obscure internal mapping table
    unordered_map<char, tuple<int,int,int,int>> mapping = {
        {'!', {3,0,1,0}},    // (cell delta, nose flip, shadow delta, ghost delta)
        {'@', {-7,1,2,1}},
        {'#', {5,0,-1,3}},
        {'$', {0,0,0,0}},    // output
        {'~', {0,1,0,0}},
        {'%', {1,0,3,-2}},
        {'^', {-3,0,0,5}},
        {'&', {2,1,-2,0}},
        {'*', {4,0,1,1}},
        {'(', {-2,1,0,-1}},
        {')', {0,0,4,0}},
        {'_', {0,0,0,1}},
        {'+', {7,0,-3,2}}
        // more can be added to increase complexity
    };

    // Randomize code positions internally for extra human-impossibility
    vector<int> idxs(code.size());
    iota(idxs.begin(), idxs.end(), 0);
    // leave original order for now; generator will randomize

    for(size_t i=0; i<code.size(); i++){
        char cmd = code[i];
        if(mapping.find(cmd) == mapping.end()) continue;
        auto [dC, flip, dS, dG] = mapping[cmd];
        
        cell = (cell + dC + 256) % 256;
        shadow = (shadow + dS + 256) % 256;
        ghost = (ghost + dG + 256) % 256;
        if(flip) nose = 1 - nose;
        
        if(cmd == '$'){
            unsigned char out = (cell + shadow + ghost) % 256;
            cout << out;
        }

        if(debug){
            cout << "\n[DEBUG] cmd=" << cmd 
                 << " cell=" << cell 
                 << " shadow=" << shadow
                 << " ghost=" << ghost
                 << " nose=" << nose << "\n";
        }
    }

    // Extra pseudo-obfuscation loop to confuse humans
    for(int i=0; i<ghost; i++){
        cell = (cell + i*i + shadow - ghost) % 256;
        shadow = (shadow + i*2 - nose) % 256;
        ghost = (ghost + i*3 + nose) % 256;
    }

    // Wrap up with random checksum (ignored by interpreter)
    int checksum = (cell + shadow + ghost + nose) % 256;
    (void)checksum;

    return 0;
}




// Difficulty: apocalyptic
// difficulty note:
// If you need me, ill be hiding in my bunker 
